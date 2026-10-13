# Lucas Translate for Omarchy

[![Native checks](https://github.com/lucaslus/lucas-translate-omarchy/actions/workflows/omarchy-native.yml/badge.svg)](https://github.com/lucaslus/lucas-translate-omarchy/actions/workflows/omarchy-native.yml)

**Omarchy 原生划词翻译、词典与本地 OCR，界面跟随系统主题。**

界面以 QML / Qt Quick 插件运行在 Omarchy Shell 中，独立的 Rust 后端负责翻译、词典、历史、凭据和本地 OCR。本仓库可以独立构建、安装和维护，无需检出原仓库或安装 Tauri 桌面版。

## 适用平台与仓库关系

| 使用环境 | 对应项目 | 技术栈 |
| --- | --- | --- |
| 提供兼容插件运行时的 Omarchy / Hyprland / Wayland 桌面 | 本仓库 `lucas-translate-omarchy` | QML / Qt Quick + Rust |
| macOS、Windows、普通 Linux 桌面 | [Lucas Translate 桌面版](https://github.com/lucaslus/luca-translate) | Tauri 2 + HTML / CSS / JavaScript + Rust |

本项目从原仓库的 Omarchy Native 部分迁出，保留了相关 Git 历史。插件 ID 仍为 `lucas.translate`，已有原生安装可通过本仓库的安装器更新。

两个仓库各自包含一份 `lucas-core` 源码；共享核心的修复需要有意识地同步。迁移来源、兼容性和维护边界见 [迁移说明](docs/MIGRATION.md)。

## 功能

- 输入翻译与划词翻译，支持语言路由和按服务重试。
- 有道词典、音标与发音，Bing、Google、DeepL 免费通道。
- DeepL 官方 API、OpenAI 兼容服务与本地 AI 服务；支持流式响应和取消。
- 本地截图 OCR：截图翻译，或将识别文字复制到剪贴板。
- 历史与收藏：搜索、复制、分页和重新翻译。
- 可调整服务顺序和桌面快捷键；窗口、字体和颜色跟随 Omarchy。
- 可选截图标注编辑器。

## 安装前准备

需要已启动且已解锁的 Omarchy 图形会话，Shell 必须提供服务、面板和栏组件插件入口，以及 `Commons`、`Ui.BorderSurface`、`Ui.Dropdown`、`Ui.TextField`、`Ui.Toggle` 等组件。安装器会检查 Shell 和活动显示器，并拒绝在锁屏状态下修改插件。

| 用途 | 依赖 |
| --- | --- |
| 安装、快捷键和剪贴板 | Python 3、`wl-clipboard`，以及可用的 `omarchy`、`omarchy-shell`、`hyprctl` |
| 从源码构建后端 | Git、Rust stable / Cargo、C/C++ 编译工具、`pkg-config`、libdbus 开发文件 |
| 截图与本地 OCR | `grim`、`slurp`、`tesseract`，英语与简体中文语言数据（`eng`、`chi_sim`） |
| 保存 API 密钥 | 已解锁的 Secret Service 密钥环 |
| 词典发音 | `mpv` |
| 截图标注 | `lucas-screenshot-editor`；构建步骤见 [开发文档](docs/DEVELOPMENT.md#optional-screenshot-editor) |

翻译服务还需要访问对应的网络服务；AI 服务的地址、模型与密钥，以及 DeepL API 的密钥，在应用设置中配置。

## 安装

### 从源码安装

准备好上述编译依赖后，在已解锁的 Omarchy 桌面中，以当前桌面用户运行：

```sh
git clone https://github.com/lucaslus/lucas-translate-omarchy.git
cd lucas-translate-omarchy
python3 scripts/install-omarchy-native.py
```

安装器会自动编译原生 Rust 后端，备份受管理的桌面文件，安装插件、后端和启动器，添加栏组件并配置六个桌面快捷键。它会验证插件发现、后端就绪、实际加载的界面版本和 Hyprland 配置；失败时尝试恢复备份。遇到快捷键冲突时，对应操作会留空，并在设置中显示原因。

安装器使用桌面用户的配置和密钥环，**请勿使用 `sudo` 运行安装器**。如需保留现有 Hyprland 快捷键配置，可使用：

```sh
python3 scripts/install-omarchy-native.py --without-shortcuts
```

已单独构建后端时，也可传入与当前源码匹配的可执行文件：

```sh
python3 scripts/install-omarchy-native.py \
  --binary target/release/lucas-translate-omarchy-backend
```

### 从预编译安装包安装

安装包可从 [Releases](https://github.com/lucaslus/lucas-translate-omarchy/releases) 获取；尚无已发布 Release 时，可打开一次成功的 [Native checks](https://github.com/lucaslus/lucas-translate-omarchy/actions/workflows/omarchy-native.yml) 运行，下载 `omarchy-native` artifact，并先解压外层 ZIP。下载 Actions artifact 需要登录 GitHub。

使用与机器架构匹配的 Linux 包；当前 CI 产物为 `x86_64`。以 `0.1.0` 包为例：

```sh
tar -xzf lucas-translate-omarchy-0.1.0-x86_64.tar.gz
cd lucas-translate-omarchy
python3 scripts/install-omarchy-native.py \
  --binary bin/lucas-translate-omarchy-backend
```

安装包包含插件、后端和安装器，无需 Rust 编译器。安装时仍需要已解锁的 Omarchy 会话和相应运行依赖。

`omarchy plugin add` 只克隆插件文件，不会安装 Rust 后端。完整安装与更新请使用上述安装器。

## 使用

安装后可通过栏组件打开，也可调用以下命令。表中的组合是初始默认值；已有自定义组合会沿用，实际分配情况以设置页为准。

| 操作 | 命令 | 默认快捷键 |
| --- | --- | --- |
| 显示面板 | `lucas-translate-native --show` | — |
| 新建输入 | `lucas-translate-native --input` | `Super+Ctrl+Shift+I` |
| 显示 / 隐藏 | `lucas-translate-native --toggle` | `Super+Ctrl+Shift+T` |
| 翻译当前选中文字 | `lucas-translate-native --selection` | `Super+Ctrl+Shift+D` |
| 截图识别并翻译 | `lucas-translate-native --screenshot` | `Super+Ctrl+Shift+S` |
| OCR 文字复制到剪贴板 | `lucas-translate-native --ocr` | `Super+Ctrl+Shift+C` |
| 截图并标注 | `lucas-translate-native --annotate` | `Super+Ctrl+Shift+P` |

输入框中 `Enter` 翻译，`Shift+Enter` 换行，`Escape` 取消正在进行的翻译并隐藏面板。`Ctrl+L` 聚焦并选中输入，`Ctrl+Up/Down` 切换结果，`Ctrl+Shift+C` 复制所选结果。

划词读取取决于来源应用的无障碍接口或 Copy 快捷键；没有有效选区时打开空输入。Copy 回退会恢复原剪贴板内容。OCR 在本机执行；截图翻译随后会把识别文字交给启用的翻译服务，`--ocr` 仅复制识别文字。

界面、输入法、划词和截图行为的完整说明见 [运行行为](docs/DEVELOPMENT.md#runtime-behavior)。

## 更新、回退与卸载

源码安装可在仓库中更新后重新运行安装器：

```sh
git pull --ff-only
python3 scripts/install-omarchy-native.py
```

安装包用户下载并解压新版包，再按安装包方式运行安装器。每次更新都使用同一版本的插件与后端，并保留原生用户数据。

安装结束会打印备份目录，默认位于 `~/.local/state/lucas-translate-omarchy/install-backups/`。回退和恢复旧桌面集成需要使用对应备份；当前没有自动卸载命令。备份文件与安装路径的对应关系、回退步骤及数据保留方式见 [回退与卸载说明](docs/MIGRATION.md#回退与卸载)。

## 安装位置与数据

下表为未自定义 XDG 环境变量时的默认路径：

| 内容 | 路径 |
| --- | --- |
| Shell 插件 | `~/.config/omarchy/plugins/lucas.translate/` |
| 后端与启动器 | `~/.local/bin/lucas-translate-omarchy-backend`、`lucas-translate-native`、`lucas-translate` |
| 配置、历史与收藏 | `~/.local/share/lucas-translate-omarchy/` |
| 诊断日志 | `~/.local/state/lucas-translate-omarchy/logs/` |
| 安装备份 | `~/.local/state/lucas-translate-omarchy/install-backups/` |

首次启动时，如果发现旧桌面版数据，会复制设置并对 SQLite 历史做快照，保留原数据。原生版使用独立的凭据命名空间；旧密钥按需复制，清除原生密钥不会删除旧桌面版凭据。诊断日志只记录允许的元数据，不记录输入、翻译内容或密钥。

`lucas-translate` 启动器在 Omarchy 图形会话中打开原生版，其他 Linux 会话转交已有的系统桌面版可执行文件。

## 源码目录与开发

```text
lucas-translate-omarchy/
├── manifest.json          # Omarchy 插件清单
├── Cargo.toml / Cargo.lock
├── qml/                   # 界面入口、组件和 JavaScript
├── assets/                # 提供商图标等资源
├── runtime/               # 已安装插件使用的启动器、桥接和快捷键
├── crates/
│   ├── lucas-core/        # 翻译与词典核心
│   └── lucas-omarchy/     # 原生 Rust 后端
├── scripts/               # 构建、安装与验证工具
├── tests/                 # Python 与 QML 测试、隔离夹具
├── packaging/editor/      # 可选截图编辑器补丁与许可说明
├── docs/                  # 开发、行为和迁移说明
└── .github/workflows/     # Linux、Qt 与发布流程
```

两个 Rust crate 使用一个根 workspace、锁文件和 `target/`；构建包位于 `dist/`。源码、安装包与安装后的插件保持相同的运行时相对路径，安装器只复制运行需要的文件。

```sh
cargo build --locked
cargo fmt --all -- --check
cargo test --package lucas-core --locked -- --skip network
cargo test --package lucas-translate-omarchy-backend --locked
bash scripts/build-omarchy-native.sh
```

原生构建与完整验证请在 Linux 上进行。CI 覆盖核心、后端、快捷键、安装回滚、打包、隔离 Wayland 剪贴板、Qt 界面交互和 QML→Rust 桥接。真实桌面的焦点、物理快捷键、区域选取、在线服务和密钥环仍需在已解锁的 Omarchy 桌面实测。测试命令与发布布局见 [开发文档](docs/DEVELOPMENT.md)。

## 许可证

本仓库使用 [MIT](LICENSE) 许可证。可选截图编辑器保留独立的 MPL-2.0 许可与补丁声明，见 [editor notices](https://github.com/lucaslus/lucas-translate-omarchy/blob/main/packaging/editor/NOTICE)。
