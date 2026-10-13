# Repository extraction

This repository was extracted from
[lucaslus/luca-translate](https://github.com/lucaslus/luca-translate) at commit
`6001d3ad6af914b3127522540bdc590c3f2faa9e` on 2026-10-13.
Relevant Git history and original authorship were retained. Commit hashes changed
because desktop-only files were excluded and plugin paths were relocated.

## Scope

- `omarchy/` became the repository root: QML, JavaScript, manifest, assets,
  runtime scripts and native tests.
- `crates/lucas-omarchy/` is the standalone Rust backend.
- `crates/lucas-core/` is a source snapshot of the shared translation core.
  It is included locally so builds require no checkout of the desktop repository.
- Native build, installation, validation and UI test scripts moved with the plugin.
- The screenshot editor build helper and its MPL-2.0 patch notices were retained.
  The editor remains an optional external executable, not a Tauri dependency.

The desktop repository retains its own `lucas-core`, Tauri host, desktop tests,
Arch packaging and legacy Hyprland integration. Core changes no longer propagate
automatically between repositories. Port shared fixes deliberately, recording
the source commit, until a separately versioned core package is introduced.

## Compatibility

The plugin ID remains `lucas.translate`. Backend executable names, launchers,
managed desktop shortcuts, data paths and credential namespaces are unchanged.
Existing native installations can update using the standalone installer; no
user-data migration or desktop-app installation is needed.

QML imports now qualify Omarchy's `Color`, `Style` and `Border` through the
`Commons` namespace. This preserves Shell theme bindings while avoiding the
`Color` singleton collision found in the Qt 6.12 CI runtime during extraction.
UI and bridge checks run as an unprivileged container user, matching the native
installer and shortcut manager's existing refusal to run as root.

`manifest.json` lives at the root. This extraction is not a marketplace
submission or a claim of marketplace approval. Installing the backend still
requires the documented installer, even when the Shell clones the QML plugin.

## Build and release

Build on Linux with `bash scripts/build-omarchy-native.sh`. The archive contains
root-level plugin files, the backend, and installer helpers. The installer stages
only runtime assets; Git metadata, Rust source, tests and build output are excluded.

The CI workflow tests the backend, core, shortcuts, transactional installation,
private Wayland transfers and QML interactions. Publishing a `v<version>` tag runs
the same checks before creating a draft release. The tag must match both the
plugin manifest and the Rust backend package version.
