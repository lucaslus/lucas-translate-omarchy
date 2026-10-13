# Development

## Repository layout

```text
lucas-translate-omarchy/
├── manifest.json              # Omarchy discovery and QML entry points
├── Cargo.toml                 # Rust workspace
├── Cargo.lock                 # Pinned dependencies for both crates
├── qml/                       # Entry points, native components and JavaScript
├── assets/logos/              # Provider icons
├── runtime/                   # Installed launchers, backend bridge and shortcuts
├── crates/
│   ├── lucas-core/            # Translation and dictionary library
│   └── lucas-omarchy/         # Headless native backend
├── scripts/                   # Build, install and verification tools
├── tests/                     # Python fixtures, integration tests and QML harnesses
├── packaging/editor/          # Optional screenshot editor patch and license notices
├── docs/                      # Development and migration notes
└── .github/workflows/         # Linux backend, Qt UI and release checks
```

The root manifest is required for Omarchy's Git-based plugin discovery.
Its entry points are safe relative paths under `qml/`, rather than root-level
QML files. Related QML and JavaScript stay in one local module so component
resolution follows the same rules in source, tests and installations.

`runtime/` contains code used by the installed plugin. `scripts/` contains
developer and installer tools; it is excluded from the installed plugin.
`scripts/plugin_files.py` is the single staging helper used by the installer,
release builder and QML test runners. It copies approved runtime files, the
manifest, README, license and documentation, preserving their relative paths.
Git metadata, Rust sources, tests, caches and build outputs are never staged.

## Rust workspace

Run Cargo from the repository root:

```sh
cargo build --locked
cargo fmt --all -- --check
cargo test --package lucas-core --locked -- --skip network
cargo test --package lucas-translate-omarchy-backend --locked
```

The default workspace member is the native backend; it uses the local core crate.
Both share the root lockfile and write build output to `target/`. Python tests
default to `target/debug/lucas-translate-omarchy-backend`. Set
`LUCAS_NATIVE_BACKEND` for UI or bridge tests using a separately built binary.
Build and test the native runtime on Linux. macOS can inspect and compile the
Rust code but cannot run the native Wayland/Quickshell UI checks.

The core remains a source snapshot shared with the desktop repository. Port
shared fixes deliberately and record their source commits; see
[Migration notes](MIGRATION.md). The screenshot editor is an optional external
executable with separate license notices, rather than a workspace member.

## Build outputs and release layout

`target/` and `dist/` are ignored. Build a Linux archive with:

```sh
bash scripts/build-omarchy-native.sh
```

The archive contains the same root manifest and `qml/`, `assets/`, `runtime/`
directories as the checkout, plus `bin/lucas-translate-omarchy-backend` and
`scripts/install-omarchy-native.py` with its staging helper. The extracted
archive can be installed without a source checkout or Rust compiler:

```sh
python3 scripts/install-omarchy-native.py --binary bin/lucas-translate-omarchy-backend
```

The installer replaces the managed plugin and backend together. Plugin ID,
user data, credentials and installed launcher names are unchanged by the source
layout. Update using a matching checkout or release archive; Omarchy's plugin
clone command alone does not install or update the Rust backend.

CI tests the backend, isolated desktop integration, packaged installation, actual
Qt components, UI interactions and QML-to-Rust bridge. QML tests stage the runtime
with the same helper used for releases, so path and asset errors are exercised
before publication. Interactive compositor focus, real shortcuts and region
selection still require an unlocked Omarchy desktop smoke test.

## Full verification

```sh
cargo build --locked
python3 -B tests/backend_test.py
python3 -B tests/stability_test.py
python3 -B tests/selection_wayland_test.py
python3 -B tests/shortcuts_test.py
python3 -B tests/integration_test.py
python3 -B scripts/test-omarchy-native.py
python3 -B scripts/test-omarchy-ui.py
python3 -B scripts/test-omarchy-bridge.py
omarchy plugin validate .
```

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

## Optional screenshot editor

Build from a source checkout on Linux. It requires Rust, Git, `pkg-config`,
GTK4, libadwaita and gtk4-layer-shell development packages. The helper fetches
a pinned Tensaku source revision and applies the separately licensed patch
in `packaging/editor/`; it does not replace the system Tensaku executable.

```sh
bash scripts/build-screenshot-editor.sh
install -Dm755 target/screenshot-editor/source/target/release/tensaku \
  "$HOME/.local/bin/lucas-screenshot-editor"
```

Translation and OCR do not require this editor.

## Runtime behavior

### Interface and theme bindings

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

### Input, selection, capture and settings

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
