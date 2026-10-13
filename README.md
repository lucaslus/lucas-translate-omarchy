# Lucas Translate for Omarchy

[![Native checks](https://github.com/lucaslus/lucas-translate-omarchy/actions/workflows/omarchy-native.yml/badge.svg)](https://github.com/lucaslus/lucas-translate-omarchy/actions/workflows/omarchy-native.yml)

**原生划词翻译、词典与本地 OCR，界面跟随 Omarchy 主题。**

独立的 Omarchy 原生仓库，基于 QML / Qt Quick 与 Rust，无 Tauri / WebView。
macOS、Windows 和普通 Linux 版本请使用 [Lucas Translate 桌面版](https://github.com/lucaslus/luca-translate)。
迁移范围、历史和共享核心的维护方式见 [Migration notes](docs/MIGRATION.md)。

Omarchy uses QML and Qt Quick inside the existing Omarchy Shell. A separate Rust
process supplies translation, dictionaries, credentials, history and local OCR.
The native build has no Tauri or WebView dependency.

The interface binds to the installed Shell's `qs.Commons.Color`, `Style` and
`Border` and uses `qs.Ui` controls. There is no application theme, Dark/Light
selector or stored native theme preference. Theme changes update the open panel
without replacing its input, results or running requests. Font sizes, spacing and
surface styling follow the Shell's effective settings, including user overrides.
All native text and controls use the desktop's `sans-serif` fontconfig alias,
including language fallbacks, rather than the Shell bar's monospace font. The
family is defined centrally in `qml/Typography.js`; no font is bundled or installed.
The compact panel is 440 logical pixels wide at the default Shell scale, with
a short empty state that expands for results. The title-free toolbar uses uniform vector icons for history, favorites and
settings, with named navigation tooltips; result sections use separators rather than nested
boxes, and duplicate dictionary paragraphs are displayed once while remaining
available for copy and favorites. Copy and favorite actions use icon buttons with
tooltips; saved favorites show a filled star. An overflow fade and downward icon
appear only while additional translations remain below the viewport; clicking the
icon advances the results and reaching the bottom hides it. Settings use compact
switches with provider toggles in a vertical list with drag handles. Secondary text retains
the theme's muted color when readable, otherwise
blending it toward the theme's popup text to reach 4.5:1 contrast when that text
color supports it. This also covers input placeholders and metadata. Language controls and the Clear/Translate icons share one row between
the input and results; Translate becomes a stop icon while a request is running. Capture actions use their existing desktop shortcuts without
adding buttons to the translation panel. AI and DeepL API forms appear only when
their switches are enabled.

## Runtime requirements

- Omarchy Quickshell plugin runtime with `Commons`, `Ui.BorderSurface`,
  `Ui.Dropdown`, `Ui.TextField`, `Ui.Toggle`, panel and service entry points.
  Verified locally on Omarchy `4.0.0.alpha` / Quickshell `0.3.1` on 2026-10-03.
- `python`, `wl-clipboard`, `grim`, `slurp`, `tesseract`,
  `tesseract-data-eng`, `tesseract-data-chi_sim`.
- An unlocked Secret Service for API keys; keys never enter IPC read responses.
- `mpv` for dictionary pronunciation; `omarchy` and `lucas-screenshot-editor`
  for the existing system screenshot/edit workflow.

## Install from this source checkout

Unlock the desktop before installing or updating the plugin. The locally
installed Shell can abort when plugin changes reload a locked session; the
installer checks the lock IPC and active compositor before modifying files.
It verifies the running panel's revision against a digest of the installed QML,
scripts and backend. If the Shell retains cached QML after an update, it restarts
the unlocked Shell once and checks the revision again before reporting success.

Clone this standalone repository, then install as your desktop user:

```sh
git clone https://github.com/lucaslus/lucas-translate-omarchy.git
cd lucas-translate-omarchy
python3 scripts/install-omarchy-native.py
```

The installer builds only `crates/lucas-omarchy`, backs up the previous desktop
configuration, installs `~/.config/omarchy/plugins/lucas.translate`, installs the
backend and launcher to `~/.local/bin`, adds the bar entry, and switches the six
existing shortcuts to the native launcher. It checks actual discovery and
backend readiness and the loaded UI revision before switching shortcuts, validates the compositor reload,
and restores managed files if installation fails. Occupied shortcuts are left
unassigned and reported in native Settings.

For a backend built separately:

```sh
python3 scripts/install-omarchy-native.py \
  --binary target/release/lucas-translate-omarchy-backend
```

`--without-shortcuts` installs the plugin without changing Hyprland bindings.
The user desktop entry and `~/.local/bin/lucas-translate` dispatch to the native
version only in an Omarchy graphical session. Other Linux sessions dispatch to
the existing system desktop executable, including Hyprland sessions without a
running Omarchy Shell. The [desktop repository](https://github.com/lucaslus/luca-translate) retains the
Tauri package and source for other platforms.

Build a standalone native bundle with `bash scripts/build-omarchy-native.sh`.
Extract its archive and run
`python3 scripts/install-omarchy-native.py --binary bin/lucas-translate-omarchy-backend`
from the extracted `lucas-translate-omarchy` directory. The bundle contains the
plugin, installer and backend; installation does not require the Tauri package
or a Rust compiler.

Update by rerunning the same installer from a newer checkout or extracted release.
Do not run it with sudo. For optional screenshot annotation, build the separately
licensed editor from a source checkout with `bash scripts/build-screenshot-editor.sh` on Linux (Rust,
Git, `pkg-config`, GTK4, libadwaita and gtk4-layer-shell development packages are required), then
install `target/screenshot-editor/source/target/release/tensaku` as
`~/.local/bin/lucas-screenshot-editor`. Translation and OCR do not require it.

## Usage

```sh
lucas-translate-native --show
lucas-translate-native --toggle
lucas-translate-native --input
lucas-translate-native --selection
lucas-translate-native --screenshot
lucas-translate-native --ocr
lucas-translate-native --annotate
```

`Enter` translates the input; `Shift+Enter` adds a newline. Escape cancels an
active translation and hides the panel. Input focus is restored on opening,
returning to Translate, selecting a language, submitting or clearing input.
An Enter used to confirm an active input-method composition remains available
to the input method; press Enter again after the text is committed. Holding Enter
does not repeatedly restart translations. The command
`omarchy-shell shell call lucas.translate health input` reports only focus,
composition and text length for diagnosing input, without exposing the query.
Outside-click or Close hides it while
retaining its input and allowing translation to finish. OCR-to-clipboard
does not open the panel or invoke translation. Cancelling a region capture
leaves the clipboard unchanged. Screenshot annotation reuses the previous system flow: Omarchy's region picker
and the installed `lucas-screenshot-editor`. Enter confirms to the clipboard and
closes; Escape, Cancel or outside-click discards the image. The editor retains its
existing save controls. Capture files are private and removed when the editor
closes, including cancellation and failures. Editing has no forced time limit.
Backend shutdown kills the editor and removes temporary files.
Selection capture requires the source application
to expose its live selection via accessibility or respond to its Copy shortcut.
The selection action pins the focused window at invocation, clears old results,
and opens an empty input panel when no selection is available. It never uses a
persisted PRIMARY offer or existing clipboard text as a query. Accessibility
reads leave the clipboard untouched; the Copy fallback accepts only a new offer
and restores all original clipboard MIME formats before translating. Terminals
use Ctrl+Shift+C. Editors known to copy a whole line without a selection require
accessibility; otherwise the action opens empty input.
Copy fallback recognizes native app IDs and executable names, including Feishu
windows with an empty app ID. Synthetic Copy uses Omarchy's explicit key-down
and delayed key-up sequence.

Settings includes service toggles, language routing, local/OpenAI-compatible AI,
DeepL API with a saved-key connection test, interface language, editable ordered
language rules, desktop shortcuts and a local diagnostics entry. Appearance is always managed
by Omarchy. History supports search and expanding all provider results. History and favorites support paging, copy, toggling saved state and retranslating; history
deletion requires a second confirmation. Dictionary text and results are rendered
as plain text, including markup-like provider content.

Language rules can be added, removed and reordered, and are saved explicitly;
unrelated settings refreshes preserve unfinished rule, shortcut and AI field edits.
The six service switches form one vertical list. Drag a row's handle to reorder;
dropping saves immediately and reorders current and future translation cards,
including single-service retries. The focused handle also supports Up/Down keys.
Cancelled drags do not save; failed saves restore the persisted order, and quick
successive moves retain the latest order. Disabled services keep their position.
History storage, retention days, record limits and OCR line cleanup are saved
explicitly. Retention defaults to unlimited and OCR cleanup is disabled by default.
AI responses stream incrementally and can be cancelled; interrupted responses
are not saved as successful history.

Within the panel, Ctrl+Up/Down selects a result, Ctrl+Shift+C copies it and Ctrl+L focuses and selects the input. Native shortcuts
use the existing managed Hyprland combinations, including saved custom values;
the settings editor currently accepts combination text instead of key recording.
Plugin enablement and startup are controlled by Omarchy Shell, replacing the
desktop application's login-autostart setting.

## Data and isolation

Native data lives in `$XDG_DATA_HOME/lucas-translate-omarchy` (default
`~/.local/share/lucas-translate-omarchy`). First startup copies existing settings
and takes a SQLite snapshot of `com.lucas.translate/lucas.db`, including committed
WAL data. Original data remains intact. Native interface preferences exclude
the old app's theme, font-size and shortcut settings. Native shortcuts are
managed through Hyprland independently. Logs contain allowlisted metadata,
not input, translations or credentials.

Native keys use a separate Secret Service namespace. Imported desktop key ids
are copied into that namespace on first use; replacing or clearing a native key
does not delete the original desktop credential.

The backend reuses `lucas-core`, including shared provider cooldowns, request slots and result caching. Its host modules were adapted from
the desktop implementation into a separate crate, so the native runtime can be
rebuilt without changing the Tauri host. Improvements common to both variants
should subsequently be extracted deliberately rather than changing the desktop
host as a side effect of native work.

macOS, Windows and ordinary Linux continue in the separate desktop repository.
This repository builds only the native backend and plugin archive. The translation
core is included as a local source snapshot; see [Migration notes](docs/MIGRATION.md)
for how shared changes are maintained. Tagging `v<version>` runs the native checks
and creates a draft GitHub release with the Linux archive and SHA-256 checksum.

## Verification

```sh
cargo test --package lucas-core --locked -- --skip network
cargo test --package lucas-translate-omarchy-backend --locked
cargo build --locked
python3 tests/backend_test.py
python3 -B tests/stability_test.py
python3 tests/shortcuts_test.py
python3 -B tests/integration_test.py
python3 scripts/test-omarchy-native.py
python3 -B scripts/test-omarchy-ui.py
python3 -B scripts/test-omarchy-bridge.py
omarchy plugin validate .
```

源码目录、Rust workspace 与发布包边界见 [Development guide](docs/DEVELOPMENT.md)。

Backend tests use a local HTTP fixture and fake clipboard/capture tools; they
do not call public providers or modify the real clipboard. The QML test loads
actual Shell components in a temporary, windowless test configuration and
checks theme changes, delayed service injection, stale-result rejection and
preservation of input/results/running requests. CI runs backend, shortcut, component, interaction and bridge checks, using a pinned Omarchy source revision for the UI modules and an Arch Qt/Quickshell runtime. The interaction runner sends mouse and keyboard events to the
actual controls in an offscreen Mesa window. The bridge runner additionally
connects the actual QML service to the Rust binary and a localhost provider.
Capture, OCR and clipboard executables are replaced with isolated fixtures.
JSON reports and logs are saved under `dist/test-results`. Run the native
backend and Quickshell checks on Linux; macOS cannot exercise this runtime.

Stability checks exercise malformed transport/provider responses, rate limits,
overload, cancellation, process shutdown, concurrent configuration writes,
system editor confirm/cancel/failure/cleanup, bounded memory/file-descriptor use, repeated
theme changes and panel lifecycles. Missing noninteractive command replies time out, pending
UI requests are bounded, and pending capture state is released after backend exit. Installer tests intercept all desktop commands and verify restoration
after validation, discovery, enablement, shortcut and final-state failures.

These tests do not establish interactive Wayland focus, real region selection,
physical keybindings, live provider/keyring access, hardware rendering or
long-running stability. Those require an unlocked desktop smoke test.

## Restore the previous desktop integration

Installation prints its backup directory under
`~/.local/state/lucas-translate-omarchy/install-backups/`. It contains the previous
plugin (if present), launcher/backend, desktop entry, `shell.json`, Hyprland Lua
and managed shortcut state. Disable `lucas.translate` and restore the appropriate
snapshot when reverting. Keep native data if you intend to reinstall.
