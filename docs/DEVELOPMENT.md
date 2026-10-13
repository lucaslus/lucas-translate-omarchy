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
Build and test the desktop runtime on Linux. macOS can inspect and compile the
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
