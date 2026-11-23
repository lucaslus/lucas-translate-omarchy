//! Read live selections; never use PRIMARY or a pre-existing clipboard offer.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;

const LIMIT: usize = 80_000;
type Reference = (String, zbus::zvariant::OwnedObjectPath);

#[derive(Deserialize, Serialize)]
struct Offer {
    text: Option<String>,
    error: Option<String>,
}

pub fn offer() -> Result<(), String> {
    let mime = std::env::var("CLIPBOARD_TYPE").unwrap_or_default();
    let mut bytes = Vec::new();
    std::io::stdin()
        .take((LIMIT + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read selection")?;
    let result = if bytes.len() > LIMIT {
        Offer {
            text: None,
            error: Some("Select 1–20000 characters".into()),
        }
    } else if mime.is_empty()
        || mime.starts_with("text/")
        || ["UTF8_STRING", "STRING", "TEXT"].contains(&mime.as_str())
    {
        match String::from_utf8(bytes) {
            Ok(text) => Offer {
                text: Some(text),
                error: None,
            },
            Err(_) => Offer {
                text: None,
                error: Some("Selection is not UTF-8 text".into()),
            },
        }
    } else {
        Offer {
            text: None,
            error: None,
        }
    };
    println!(
        "{}",
        serde_json::to_string(&result).map_err(|_| "Cannot encode selection")?
    );
    Ok(())
}

/// Runs in a separate clipboard owner process, like wl-copy. Every MIME format
/// is restored, and the owner exits when the user next replaces the clipboard.
pub fn restore() -> Result<(), String> {
    use wl_clipboard_rs::copy::{MimeSource, MimeType, Options, Source};
    let sources: Vec<(String, Vec<u8>)> =
        serde_json::from_reader(std::io::stdin().take(100_000_000))
            .map_err(|_| "Invalid clipboard snapshot")?;
    let mut options = Options::new();
    options
        .foreground(true)
        .omit_additional_text_mime_types(true);
    let prepared = options
        .prepare_copy_multi(
            sources
                .into_iter()
                .map(|(mime, bytes)| MimeSource {
                    source: Source::Bytes(bytes.into()),
                    mime_type: MimeType::Specific(mime),
                })
                .collect(),
        )
        .map_err(|_| "Cannot restore clipboard")?;
    println!("ready");
    std::io::stdout()
        .flush()
        .map_err(|_| "Cannot acknowledge clipboard restore")?;
    prepared
        .serve()
        .map_err(|_| "Cannot serve restored clipboard".to_string())
}

async fn command(args: &[&str]) -> Result<std::process::Output, String> {
    let (name, arguments) = args.split_first().ok_or("Missing desktop command")?;
    tokio::time::timeout(
        Duration::from_millis(600),
        Command::new(name)
            .args(arguments)
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .output(),
    )
    .await
    .map_err(|_| "Selection operation timed out")?
    .map_err(|_| "Selection tool unavailable".to_string())
}

async fn focused() -> Option<Value> {
    let output = command(&["hyprctl", "-j", "activewindow"]).await.ok()?;
    if !output.status.success() {
        return None;
    }
    serde_json::from_slice(&output.stdout).ok()
}

fn same_window(left: &Value, right: &Value) -> bool {
    left["address"]
        .as_str()
        .is_some_and(|address| !address.is_empty() && right["address"].as_str() == Some(address))
        && left["pid"]
            .as_u64()
            .is_some_and(|pid| pid > 0 && right["pid"].as_u64() == Some(pid))
}

/// None means the app exposes no usable accessibility tree. Some(None) means
/// it does expose live text but currently has no selection; do not synthesize
/// Copy in that case (editors may copy the whole line with no selection).
async fn accessible_selection(pid: u32) -> Option<Option<String>> {
    let session = zbus::Connection::session().await.ok()?;
    let bus = zbus::Proxy::new(&session, "org.a11y.Bus", "/org/a11y/bus", "org.a11y.Bus")
        .await
        .ok()?;
    let address: String = bus.call("GetAddress", &()).await.ok()?;
    let connection = zbus::connection::Builder::address(address.as_str())
        .ok()?
        .build()
        .await
        .ok()?;
    accessible_selection_from(&connection, pid).await
}

async fn accessible_selection_from(
    connection: &zbus::Connection,
    pid: u32,
) -> Option<Option<String>> {
    let registry = zbus::Proxy::new(
        connection,
        "org.a11y.atspi.Registry",
        "/org/a11y/atspi/accessible/root",
        "org.a11y.atspi.Accessible",
    )
    .await
    .ok()?;
    let applications: Vec<Reference> = registry.call("GetChildren", &()).await.ok()?;
    let daemon = zbus::fdo::DBusProxy::new(connection).await.ok()?;
    let mut nodes = Vec::new();
    for application in applications {
        let name = zbus::names::BusName::try_from(application.0.as_str()).ok()?;
        if daemon.get_connection_unix_process_id(name).await.ok() == Some(pid) {
            nodes.push((application, false))
        }
    }
    if nodes.is_empty() {
        return None;
    }
    let mut text_available = false;
    let mut visited = 0;
    while let Some(((name, path), active_parent)) = nodes.pop() {
        visited += 1;
        if visited > 300 {
            return None;
        }
        let accessible = zbus::Proxy::new(
            connection,
            name.as_str(),
            path.as_str(),
            "org.a11y.atspi.Accessible",
        )
        .await
        .ok()?;
        let state: Vec<u32> = accessible.call("GetState", &()).await.ok()?;
        let flags = state.first().copied().unwrap_or(0);
        // ACTIVE=1, DEFUNCT=6, FOCUSED=12, SHOWING=25 (stable AT-SPI enum).
        if flags & (1 << 6) != 0 {
            continue;
        }
        let active = active_parent
            || flags & (1 << 12) != 0
            || (flags & (1 << 1) != 0 && path.as_str() != "/org/a11y/atspi/accessible/root");
        let interfaces: Vec<String> = accessible.call("GetInterfaces", &()).await.ok()?;
        if active
            && flags & (1 << 25) != 0
            && interfaces
                .iter()
                .any(|value| value == "org.a11y.atspi.Text")
        {
            if flags & (1 << 12) != 0 {
                text_available = true;
            }
            let text = zbus::Proxy::new(
                connection,
                name.as_str(),
                path.as_str(),
                "org.a11y.atspi.Text",
            )
            .await
            .ok()?;
            let count: i32 = text.call("GetNSelections", &()).await.ok()?;
            if count > 0 {
                let (start, end): (i32, i32) = text.call("GetSelection", &(0_i32)).await.ok()?;
                if start >= 0 && end > start && end - start <= 20_000 {
                    let selected: String = text.call("GetText", &(start, end)).await.ok()?;
                    if !selected.trim().is_empty() {
                        return Some(Some(selected));
                    }
                }
            }
        }
        let children: Vec<Reference> = accessible.call("GetChildren", &()).await.ok()?;
        nodes.extend(children.into_iter().map(|child| (child, active)));
    }
    text_available.then_some(None)
}

async fn snapshot() -> Result<Vec<(String, Vec<u8>)>, String> {
    let output = command(&["wl-paste", "--list-types"]).await?;
    if !output.status.success() {
        if String::from_utf8_lossy(&output.stderr).contains("Nothing is copied") {
            return Ok(Vec::new());
        }
        return Err("Cannot preserve clipboard formats".into());
    }
    let types = String::from_utf8(output.stdout).map_err(|_| "Invalid clipboard types")?;
    if types.lines().count() > 32 {
        return Err("Clipboard has too many formats".into());
    }
    let mut sources = Vec::new();
    let mut total = 0;
    for mime in types.lines().filter(|mime| !mime.is_empty()) {
        let mut child = Command::new("wl-paste")
            .args(["--no-newline", "--type", mime])
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| "Cannot preserve clipboard")?;
        let mut bytes = Vec::new();
        let pipe = child.stdout.take().ok_or("Cannot preserve clipboard")?;
        let status = tokio::time::timeout(Duration::from_millis(600), async {
            pipe.take((16_000_001 - total) as u64)
                .read_to_end(&mut bytes)
                .await?;
            if total + bytes.len() > 16_000_000 {
                child.kill().await?
            }
            child.wait().await
        })
        .await
        .map_err(|_| "Clipboard read timed out")?
        .map_err(|_| "Cannot preserve clipboard")?;
        total += bytes.len();
        if total > 16_000_000 || !status.success() {
            return Err("Cannot preserve clipboard formats".into());
        }
        sources.push((mime.to_string(), bytes));
    }
    Ok(sources)
}

async fn restore_snapshot(sources: Vec<(String, Vec<u8>)>) -> Result<(), String> {
    if sources.is_empty() {
        let output = command(&["wl-copy", "--clear"]).await?;
        return output
            .status
            .success()
            .then_some(())
            .ok_or("Cannot restore empty clipboard".into());
    }
    let mut file = tempfile::tempfile().map_err(|_| "Cannot preserve clipboard")?;
    serde_json::to_writer(&mut file, &sources).map_err(|_| "Cannot preserve clipboard")?;
    use std::io::{Seek, SeekFrom};
    file.seek(SeekFrom::Start(0))
        .map_err(|_| "Cannot preserve clipboard")?;
    let mut child = Command::new(std::env::current_exe().map_err(|_| "Cannot restore clipboard")?)
        .arg("--restore-selection-clipboard")
        .stdin(file)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "Cannot restore clipboard")?;
    let mut reader = BufReader::new(child.stdout.take().ok_or("Cannot restore clipboard")?);
    let mut line = String::new();
    let ready = tokio::time::timeout(Duration::from_millis(600), reader.read_line(&mut line)).await;
    if matches!(ready, Ok(Ok(_))) && line.trim() == "ready" {
        tokio::spawn(async move {
            let _ = child.wait().await;
        });
        Ok(())
    } else {
        let _ = child.kill().await;
        Err("Cannot restore clipboard".into())
    }
}

fn is_terminal_name(name: &str) -> bool {
    let lower = name.trim().to_ascii_lowercase();
    [
        "ghostty",
        "com.mitchellh.ghostty",
        "alacritty",
        "kitty",
        "foot",
        "footclient",
        "org.wezfurlong.wezterm",
        "wezterm",
        "wezterm-gui",
        "org.kde.konsole",
        "konsole",
        "terminator",
        "tilix",
        "gnome-terminal",
        "gnome-terminal-server",
        "xterm",
        "urxvt",
        "st",
        "rio",
        "contour",
    ]
    .contains(&lower.as_str())
}

fn copy_modifiers(class: &str) -> Option<&'static str> {
    let trimmed = class.trim();
    if trimmed.is_empty() {
        return None;
    }
    if is_terminal_name(trimmed) {
        Some("CTRL+SHIFT")
    } else {
        Some("CTRL")
    }
}

fn window_copy_modifiers(
    window: &Value,
    executable: Option<&std::path::Path>,
) -> Option<&'static str> {
    if window["tags"].as_array().is_some_and(|tags| {
        tags.iter()
            .filter_map(Value::as_str)
            .any(|tag| tag.trim_end_matches('*') == "terminal")
    }) {
        return Some("CTRL+SHIFT");
    }
    let class = window["class"].as_str().unwrap_or("");
    let exe_name = executable
        .and_then(std::path::Path::file_name)
        .and_then(|name| name.to_str())
        .unwrap_or("");
    if is_terminal_name(class) || is_terminal_name(exe_name) {
        return Some("CTRL+SHIFT");
    }
    copy_modifiers(class)
        .or_else(|| copy_modifiers(exe_name))
        .or_else(|| {
            window["pid"]
                .as_u64()
                .filter(|&pid| pid > 0)
                .map(|_| "CTRL")
        })
}

fn vscode_editor_data_is_empty_selection(bytes: &[u8]) -> bool {
    serde_json::from_slice::<Value>(bytes)
        .ok()
        .and_then(|value| value.get("isFromEmptySelection")?.as_bool())
        == Some(true)
}

fn read_chromium_pickle_u16_string(bytes: &[u8], offset: &mut usize) -> Option<String> {
    let header = bytes.get(*offset..offset.checked_add(4)?)?;
    let char_len = u32::from_le_bytes(header.try_into().ok()?) as usize;
    let byte_len = char_len.checked_mul(2)?;
    if byte_len > 64 * 1024 {
        return None;
    }
    let start = offset.checked_add(4)?;
    let end = start.checked_add(byte_len)?;
    let raw = bytes.get(start..end)?;
    let padded = (byte_len + 3) & !3;
    *offset = start.checked_add(padded)?;
    if *offset > bytes.len() {
        return None;
    }
    let units: Vec<u16> = raw
        .chunks_exact(2)
        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
        .collect();
    String::from_utf16(&units).ok()
}

/// Chromium/Electron packages custom clipboard MIME entries such as `vscode-editor-data`
/// inside a Pickle map under `chromium/x-web-custom-data` on Linux.
fn chromium_custom_data_is_empty_selection(bytes: &[u8]) -> bool {
    let Some(size_bytes) = bytes.get(0..4) else {
        return false;
    };
    let payload_size = u32::from_le_bytes(size_bytes.try_into().unwrap()) as usize;
    let Some(body) = bytes
        .get(4..)
        .and_then(|rest| rest.get(..payload_size.min(rest.len())))
    else {
        return false;
    };
    let Some(count_bytes) = body.get(0..4) else {
        return false;
    };
    let count = u32::from_le_bytes(count_bytes.try_into().unwrap()) as usize;
    if count > 64 {
        return false;
    }
    let mut offset = 4;
    for _ in 0..count {
        let Some(key) = read_chromium_pickle_u16_string(body, &mut offset) else {
            return false;
        };
        let Some(value) = read_chromium_pickle_u16_string(body, &mut offset) else {
            return false;
        };
        if key == "vscode-editor-data" {
            return vscode_editor_data_is_empty_selection(value.as_bytes());
        }
    }
    false
}

async fn is_empty_editor_selection(types: &str) -> bool {
    if types.lines().any(|mime| mime == "vscode-editor-data") {
        if let Ok(output) =
            command(&["wl-paste", "--no-newline", "--type", "vscode-editor-data"]).await
        {
            if output.status.success() && vscode_editor_data_is_empty_selection(&output.stdout) {
                return true;
            }
        }
    }
    if types
        .lines()
        .any(|mime| mime == "chromium/x-web-custom-data")
    {
        if let Ok(output) = command(&[
            "wl-paste",
            "--no-newline",
            "--type",
            "chromium/x-web-custom-data",
        ])
        .await
        {
            if output.status.success() && chromium_custom_data_is_empty_selection(&output.stdout) {
                return true;
            }
        }
    }
    false
}

pub async fn read(source: &Value) -> Result<Option<String>, String> {
    let Some(window) = focused().await else {
        return Ok(None);
    };
    if !source.is_null() && !same_window(source, &window) {
        return Ok(None);
    }
    let pid = window["pid"]
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .unwrap_or(0);
    if let Ok(Some(selection)) =
        tokio::time::timeout(Duration::from_millis(350), accessible_selection(pid)).await
    {
        if !focused()
            .await
            .is_some_and(|current| same_window(&window, &current))
        {
            return Ok(None);
        }
        return Ok(selection);
    }
    let executable = std::fs::read_link(format!("/proc/{pid}/exe")).ok();
    let Some(mods) = window_copy_modifiers(&window, executable.as_deref()) else {
        return Ok(None);
    };
    let address = window["address"].as_str().unwrap_or("");
    if !address.starts_with("0x")
        || !address[2..].chars().all(|c| c.is_ascii_hexdigit())
        || address.len() <= 2
    {
        return Ok(None);
    }
    let sources = tokio::time::timeout(Duration::from_millis(1500), snapshot())
        .await
        .map_err(|_| "Clipboard snapshot timed out")??;
    let mut watcher = Command::new("wl-paste")
        .args(["--no-newline", "--watch"])
        .arg(std::env::current_exe().map_err(|_| "Cannot observe selection")?)
        .arg("--selection-offer")
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "Cannot observe selection")?;
    let mut reader =
        BufReader::new(watcher.stdout.take().ok_or("Cannot observe selection")?).lines();
    // The first event is always the existing offer (including nil). It is a
    // readiness barrier only; its text must never become a query.
    tokio::time::timeout(Duration::from_millis(600), reader.next_line())
        .await
        .map_err(|_| "Selection observer timed out")?
        .map_err(|_| "Cannot observe selection")?
        .ok_or("Selection observer exited")?;
    if !focused()
        .await
        .is_some_and(|current| same_window(&window, &current))
    {
        return Ok(None);
    }
    // Follow Omarchy's universal-copy implementation: explicit down/up avoids
    // stuck synthetic modifiers and missing release events in send_shortcut.
    let dispatch = format!("function() hl.dispatch(hl.dsp.send_key_state({{ mods = \"{mods}\", key = \"C\", state = \"down\", window = \"address:{address}\" }})); hl.timer(function() hl.dispatch(hl.dsp.send_key_state({{ mods = \"{mods}\", key = \"C\", state = \"up\", window = \"address:{address}\" }})) end, {{ timeout = 50, type = \"oneshot\" }}) end");
    let sent = command(&["hyprctl", "dispatch", &dispatch]).await?;
    if !sent.status.success() {
        return Err("Cannot request current selection".into());
    }
    let event = tokio::time::timeout(Duration::from_millis(400), reader.next_line()).await;
    let _ = watcher.kill().await;
    let _ = watcher.wait().await;
    let Ok(Ok(Some(event))) = event else {
        return Ok(None);
    };
    // CLIPBOARD_TYPE is absent in some wl-clipboard watch implementations.
    // Validate the new offer's types before treating its bytes as text.
    let current_types = command(&["wl-paste", "--list-types"]).await;
    let empty_editor_copy = match &current_types {
        Ok(output) if output.status.success() => {
            let types = String::from_utf8_lossy(&output.stdout);
            is_empty_editor_selection(&types).await
        }
        _ => false,
    };
    // Restore every clipboard representation before submitting any translation.
    restore_snapshot(sources).await?;
    if empty_editor_copy {
        return Ok(None);
    }
    let current_types = current_types?;
    if !current_types.status.success()
        || !String::from_utf8_lossy(&current_types.stdout)
            .lines()
            .any(|mime| {
                mime.starts_with("text/") || ["UTF8_STRING", "STRING", "TEXT"].contains(&mime)
            })
    {
        return Ok(None);
    }
    let offer: Offer = serde_json::from_str(&event).map_err(|_| "Invalid selection offer")?;
    if let Some(error) = offer.error {
        return Err(error);
    }
    if !focused()
        .await
        .is_some_and(|current| same_window(&window, &current))
    {
        return Ok(None);
    }
    Ok(offer.text.filter(|text| !text.trim().is_empty()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct Accessible {
        child: Option<Reference>,
        text: bool,
    }
    #[zbus::interface(name = "org.a11y.atspi.Accessible")]
    impl Accessible {
        fn get_children(&self) -> Vec<Reference> {
            self.child.iter().cloned().collect()
        }
        fn get_interfaces(&self) -> Vec<String> {
            if self.text {
                vec!["org.a11y.atspi.Text".into()]
            } else {
                Vec::new()
            }
        }
        fn get_state(&self) -> Vec<u32> {
            vec![if self.text { (1 << 12) | (1 << 25) } else { 0 }, 0]
        }
    }
    struct Text {
        selected: Arc<Mutex<Option<String>>>,
    }
    #[zbus::interface(name = "org.a11y.atspi.Text")]
    impl Text {
        fn get_n_selections(&self) -> i32 {
            i32::from(self.selected.lock().unwrap().is_some())
        }
        fn get_selection(&self, _index: i32) -> (i32, i32) {
            (
                0,
                self.selected
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map_or(0, |text| text.chars().count() as i32),
            )
        }
        fn get_text(&self, _start: i32, _end: i32) -> String {
            self.selected.lock().unwrap().clone().unwrap_or_default()
        }
    }

    #[test]
    fn accessibility_reads_live_selection_without_any_clipboard_offer() {
        // The production reader owns its own process/runtime. Exercise the
        // private accessibility service with the same process isolation.
        if std::env::var_os("LUCAS_A11Y_TEST_CHILD").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "selection::tests::accessibility_reads_live_selection_without_any_clipboard_offer", "--nocapture"])
                .env("LUCAS_A11Y_TEST_CHILD", "1").output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                tokio::time::timeout(Duration::from_secs(10), accessibility_fixture())
                    .await
                    .expect("Accessibility fixture timed out");
            });
    }

    async fn accessibility_fixture() {
        use std::io::BufRead;
        struct Bus(std::process::Child);
        impl Drop for Bus {
            fn drop(&mut self) {
                let _ = self.0.kill();
                let _ = self.0.wait();
            }
        }
        let mut bus = Bus(std::process::Command::new("dbus-daemon")
            .args(["--session", "--nofork", "--print-address"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap());
        let mut address = String::new();
        std::io::BufReader::new(bus.0.stdout.take().unwrap())
            .read_line(&mut address)
            .unwrap();
        let server = zbus::connection::Builder::address(address.trim())
            .unwrap()
            .name("org.a11y.atspi.Registry")
            .unwrap()
            .build()
            .await
            .unwrap();
        let name = server.unique_name().unwrap().to_string();
        let root = "/org/a11y/atspi/accessible/root";
        let text = "/org/a11y/atspi/accessible/text";
        let selected = Arc::new(Mutex::new(None));
        server
            .object_server()
            .at(
                root,
                Accessible {
                    child: Some((name.clone(), text.try_into().unwrap())),
                    text: false,
                },
            )
            .await
            .unwrap();
        server
            .object_server()
            .at(
                text,
                Accessible {
                    child: None,
                    text: true,
                },
            )
            .await
            .unwrap();
        server
            .object_server()
            .at(
                text,
                Text {
                    selected: selected.clone(),
                },
            )
            .await
            .unwrap();
        let client = zbus::connection::Builder::address(address.trim())
            .unwrap()
            .build()
            .await
            .unwrap();
        let pid = std::process::id();
        assert_eq!(accessible_selection_from(&client, pid).await, Some(None));
        *selected.lock().unwrap() = Some("实时选区".into());
        assert_eq!(
            accessible_selection_from(&client, pid).await,
            Some(Some("实时选区".into()))
        );
        *selected.lock().unwrap() = None;
        assert_eq!(accessible_selection_from(&client, pid).await, Some(None));
        assert_eq!(accessible_selection_from(&client, pid + 1).await, None);
    }
    #[test]
    fn pins_address_and_process_not_just_app_class() {
        let window = serde_json::json!({"address":"0x123","pid":7,"class":"chromium"});
        assert!(same_window(&window, &window));
        assert!(!same_window(
            &window,
            &serde_json::json!({"address":"0x124","pid":7})
        ));
        assert!(!same_window(
            &window,
            &serde_json::json!({"address":"0x123","pid":8})
        ));
        assert!(!same_window(&Value::Null, &Value::Null));
    }
    #[test]
    fn terminal_copy_does_not_interrupt_the_shell() {
        assert_eq!(copy_modifiers("com.mitchellh.ghostty"), Some("CTRL+SHIFT"));
        assert_eq!(copy_modifiers("chromium"), Some("CTRL"));
        assert_eq!(copy_modifiers("antigravity"), Some("CTRL"));
        assert_eq!(copy_modifiers("wechat"), Some("CTRL"));
        assert_eq!(copy_modifiers("TelegramDesktop"), Some("CTRL"));
        assert_eq!(copy_modifiers("Code"), Some("CTRL"));
        assert_eq!(copy_modifiers(""), None);
    }
    #[test]
    fn copy_recognizes_all_desktop_windows_and_terminal_tags() {
        let empty = serde_json::json!({"class":""});
        assert_eq!(
            window_copy_modifiers(
                &empty,
                Some(std::path::Path::new("/opt/bytedance/feishu/feishu"))
            ),
            Some("CTRL")
        );
        assert_eq!(copy_modifiers("chatgpt"), Some("CTRL"));
        assert_eq!(
            window_copy_modifiers(
                &empty,
                Some(std::path::Path::new("/opt/Antigravity/antigravity"))
            ),
            Some("CTRL")
        );
        assert_eq!(
            window_copy_modifiers(&empty, Some(std::path::Path::new("/usr/bin/ghostty"))),
            Some("CTRL+SHIFT")
        );
        assert_eq!(
            window_copy_modifiers(
                &serde_json::json!({"class":"unknown-terminal", "tags":["terminal*"]}),
                None
            ),
            Some("CTRL+SHIFT")
        );
        assert_eq!(
            window_copy_modifiers(&serde_json::json!({"class":"", "pid":12345}), None),
            Some("CTRL")
        );
        assert_eq!(window_copy_modifiers(&empty, None), None);
    }
    #[test]
    fn detects_vscode_and_chromium_empty_selection_line_copy() {
        assert!(vscode_editor_data_is_empty_selection(
            br#"{"version":1,"isFromEmptySelection":true,"mode":"rust"}"#
        ));
        assert!(!vscode_editor_data_is_empty_selection(
            br#"{"version":1,"isFromEmptySelection":false,"mode":"rust"}"#
        ));
        assert!(!vscode_editor_data_is_empty_selection(b"not-json"));

        fn encode_pickle(entries: &[(&str, &str)]) -> Vec<u8> {
            let mut body = Vec::new();
            body.extend_from_slice(&(entries.len() as u32).to_le_bytes());
            for (k, v) in entries {
                for s in [k, v] {
                    let units: Vec<u16> = s.encode_utf16().collect();
                    body.extend_from_slice(&(units.len() as u32).to_le_bytes());
                    for u in units {
                        body.extend_from_slice(&u.to_le_bytes());
                    }
                    while body.len() % 4 != 0 {
                        body.push(0);
                    }
                }
            }
            let mut out = (body.len() as u32).to_le_bytes().to_vec();
            out.extend_from_slice(&body);
            out
        }

        let empty_sel = encode_pickle(&[
            ("other", "x"),
            (
                "vscode-editor-data",
                r#"{"version":1,"isFromEmptySelection":true,"mode":"plaintext"}"#,
            ),
        ]);
        assert!(chromium_custom_data_is_empty_selection(&empty_sel));

        let real_sel = encode_pickle(&[(
            "vscode-editor-data",
            r#"{"version":1,"isFromEmptySelection":false,"mode":"plaintext"}"#,
        )]);
        assert!(!chromium_custom_data_is_empty_selection(&real_sel));
        assert!(!chromium_custom_data_is_empty_selection(&[0, 1, 2]));
    }
}
