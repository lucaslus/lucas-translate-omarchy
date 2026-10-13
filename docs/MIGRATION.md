# Repository extraction

This repository was extracted from
[lucaslus/luca-translate](https://github.com/lucaslus/luca-translate) at commit
`6001d3ad6af914b3127522540bdc590c3f2faa9e` on 2026-10-13.
Relevant Git history and original authorship were retained. Commit hashes changed
because desktop-only files were excluded and plugin paths were relocated.

## Scope

- The original `omarchy/` plugin was extracted with its relevant history.
  The manifest stays at the root, with QML and JavaScript in `qml/`, provider
  images in `assets/`, and runtime integration in `runtime/`.
- `crates/lucas-omarchy/` is the standalone Rust backend.
- `crates/lucas-core/` is a source snapshot of the shared translation core.
  It is included locally so builds require no checkout of the desktop repository.
- Native build, installation, validation and UI test scripts moved with the plugin.
- Both Rust crates belong to one root workspace with a single `Cargo.lock`
  and `target/`. The backend's dependency versions were retained.
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
the root manifest, `qml/`, `assets/`, `runtime/`, the backend, and installer helpers.
The runtime layout is identical in the checkout, archive and installed plugin;
there is no flattened second copy of the source. The installer stages
only runtime assets; Git metadata, Rust source, tests and build output are excluded.

The CI workflow tests the backend, core, shortcuts, transactional installation,
private Wayland transfers and QML interactions. Publishing a `v<version>` tag runs
the same checks before creating a draft release. The tag must match both the
plugin manifest and the Rust backend package version.

See [Development guide](DEVELOPMENT.md) for the current source layout.

## 回退与卸载

安装器失败时会尝试自动恢复本次安装前受管理的文件。成功安装后的回退或卸载
目前需要手动处理；没有自动卸载命令。请使用安装时打印的备份目录，而非假设
最后一个备份就是你想恢复的版本。

备份目录默认在 `~/.local/state/lucas-translate-omarchy/install-backups/<timestamp>/`。
它只包含安装前已经存在的受管理文件。以下路径按默认 XDG 目录列出；自定义了
`XDG_CONFIG_HOME`、`XDG_DATA_HOME` 或 `XDG_STATE_HOME` 时，以实际目录为准。

| 备份中的名称 | 对应安装路径 |
| --- | --- |
| `plugin/` | `~/.config/omarchy/plugins/lucas.translate/` |
| `backend` | `~/.local/bin/lucas-translate-omarchy-backend` |
| `launcher` | `~/.local/bin/lucas-translate-native` |
| `dispatch` | `~/.local/bin/lucas-translate` |
| `desktop` | `~/.local/share/applications/lucas-translate.desktop` |
| `shell.json` | `~/.config/omarchy/shell.json` |
| `hyprland.lua` | `~/.config/hypr/hyprland.lua` |
| `lucas-translate.lua` | `~/.config/hypr/lucas-translate.lua` |
| `shortcuts.json` | `~/.config/hypr/lucas-translate-shortcuts.json` |

1. 解锁桌面，并暂停原生插件：`omarchy plugin disable lucas.translate`。
2. 另存当前配置，核对选定备份。回退旧原生版本时，成套恢复该快照中的插件、
   后端、启动器和桌面配置；恢复原桌面集成时，使用安装原生版之前的快照。
3. 按表恢复备份内容。某项备份不存在表示对应路径在安装前不存在；确认当前
   路径仍属于该次安装创建的文件后，移除它以恢复原来的缺失状态。安装后另有
   桌面配置修改时，合并相关设置，避免覆盖这些后续修改。
4. 完成文件恢复后，让 Shell 与 Hyprland 重新读取配置：

   ```sh
   omarchy-shell shell rescanPlugins
   omarchy-shell shell reloadConfig
   hyprctl reload
   ```

仅暂停功能时，执行第 1 步即可。若要移除原生安装并保留旧桌面集成，请恢复
安装原生版之前的快照；否则直接删除共享的 `lucas-translate` 启动器或 Hyprland
文件可能同时移除原有的桌面集成。

用户数据和凭据不在上述安装备份中。回退或卸载时保留
`~/.local/share/lucas-translate-omarchy/` 即可保留原生配置、历史和收藏；诊断日志
位于 `~/.local/state/lucas-translate-omarchy/logs/`。如要清除数据，先单独备份，
并在停用插件后自行清理这些原生目录。旧桌面版的 `com.lucas.translate` 数据
和凭据应保留；原生密钥可在停用前从设置中清除。
