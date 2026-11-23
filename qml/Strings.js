.pragma library

var labels = {
    reorder: ["Drag to reorder", "拖拽排序"],
    removed: ["Removed", "已移除"], search: ["Search records…", "搜索记录…"], viewResults: ["View results", "查看完整结果"], collapse: ["Collapse", "收起"],
    serviceOrder: ["Service order", "服务顺序"], historyStorage: ["Save history", "保存历史"], historyDays: ["Retention", "保留时间"], historyLimit: ["Record limit", "记录上限"], unlimited: ["Unlimited", "不限"], ocrCleanup: ["Join OCR text lines", "整理 OCR 断行"], usage: ["Usage", "使用偏好"],
    translate: ["Translate", "翻译"], settings: ["Settings", "设置"], history: ["History", "历史"],
    favorites: ["Favorites", "收藏"], close: ["Close", "关闭"], input: ["Type or paste text…", "输入或粘贴文字…"],
    source: ["Source", "原文"], target: ["Target", "译文"], auto: ["Automatic", "自动"],
    selection: ["Selection", "划词"], screenshot: ["Screenshot", "截图翻译"], ocr: ["OCR to clipboard", "OCR 复制"],
    annotate: ["Annotate", "截图标注"], cancel: ["Cancel", "取消"], clear: ["Clear", "清空"],
    loading: ["Translating…", "翻译中…"], empty: ["Select text, capture a region, or enter a query.", "划词、框选截图，或输入文字开始翻译。"],
    enterHint: ["Enter to translate · Shift+Enter for a new line", "回车翻译 · Shift+Enter 换行"],
    copy: ["Copy", "复制"], copied: ["Copied", "已复制"], favorite: ["Favorite", "收藏"], saved: ["Saved", "已保存"],
    retry: ["Retry", "重试"], remove: ["Remove", "移除"], save: ["Save", "保存"], more: ["Load more", "加载更多"],
    moreResults: ["More translations below", "下方还有译文"],
    language: ["Interface language", "界面语言"], system: ["System", "跟随系统"],
    theme: ["Appearance follows Omarchy’s current theme and display settings.", "外观跟随 Omarchy 当前主题与显示设置。"],
    services: ["Translation services", "翻译服务"], ai: ["AI translation", "AI 翻译"], official: ["DeepL API", "DeepL 官方 API"],
    endpoint: ["Base URL", "服务地址"], model: ["Model", "模型"], apiKey: ["API key", "API 密钥"],
    keyStored: ["Saved", "已保存"], keyEmpty: ["Optional", "可选"],
    deleteKey: ["Clear saved key", "清除已存密钥"], pro: ["DeepL Pro account", "DeepL Pro 账户"],
    routing: ["Automatic target language", "自动目标语言"], fallback: ["Fallback target", "默认目标语言"],
    fromEnglish: ["English →", "英文 →"], fromChinese: ["Chinese →", "中文 →"],
    addRule: ["Add rule", "添加规则"], moveUp: ["Move up", "上移"], moveDown: ["Move down", "下移"],
    routingHelp: ["Rules match in order; unmatched languages use the fallback target.", "按顺序匹配规则；未命中的语言使用默认目标。"],
    testConnection: ["Test connection", "测试连接"], connected: ["Connected", "连接正常"],
    saveBeforeTest: ["Save changes before testing the stored key.", "请先保存修改，再测试已保存的密钥。"],
    about: ["About", "关于"], diagnostics: ["Local diagnostics", "本地诊断"], openLogs: ["Open log directory", "查看日志目录"],
    logsPrivate: ["Logs contain status and duration, without source text, translations or credentials.", "日志只记录状态和耗时，不记录原文、译文或密钥。"],
    logsReady: ["Logging is available", "日志可用"], logsFailed: ["Logging is unavailable or cannot write", "日志不可用或暂时无法写入"],
    logsDropped: ["Skipped events", "已跳过记录"], shellManaged: ["Startup and plugin enablement are managed by Omarchy Shell.", "启动和插件启用由 Omarchy Shell 管理。"],
    shortcuts: ["Desktop shortcuts", "桌面快捷键"], shortcutHelp: ["Enter a combination such as Super+Ctrl+Shift+I. Leave blank to disable.", "输入组合，如 Super+Ctrl+Shift+I；留空禁用。"],
    toggle: ["Show / hide", "显示 / 隐藏"], inputAction: ["New input", "新建输入"],
    noHistory: ["No records yet", "暂无记录"], clearHistory: ["Clear history", "清空历史"],
    confirmClear: ["Delete all history?", "删除全部历史记录？"],
    backend: ["Starting translation service…", "正在启动翻译服务…"], backendFailed: ["Translation service stopped. Retry to reconnect.", "翻译服务已停止，重试以重新连接。"],
    interrupted: ["Translation interrupted; retry the unfinished services.", "翻译已中断，请重试未完成渠道。"],
    unavailable: ["Service unavailable", "服务不可用"], detected: ["Detected", "检测语言"], pronunciation: ["Pronunciation", "发音"],
    uk: ["UK", "英"], us: ["US", "美"],
    timeout: ["The operation timed out. Retry when the service responds.", "操作超时，请待服务恢复后重试。"],
    overloaded: ["Too many pending operations. Wait and retry.", "待处理操作过多，请稍后重试。"],
    english: ["English", "英语"], chinese: ["Simplified Chinese", "简体中文"], traditional: ["Traditional Chinese", "繁体中文"],
    japanese: ["Japanese", "日语"], korean: ["Korean", "韩语"], french: ["French", "法语"],
    german: ["German", "德语"], russian: ["Russian", "俄语"], spanish: ["Spanish", "西班牙语"],
    pen: ["Pen", "画笔"], arrow: ["Arrow", "箭头"], rectangle: ["Box", "方框"], ellipse: ["Circle", "圈画"],
    textTool: ["Text", "文字"], undo: ["Undo", "撤销"], accent: ["Accent", "强调色"],
    urgent: ["Highlight", "醒目色"], ink: ["Text color", "文字色"], annotationText: ["Label to place on the image", "在图片上放置的文字"],
    exportFailed: ["Cannot export the annotated image", "无法导出标注图片"]
}

function text(key, language) {
    var value = labels[key]
    return value ? value[language === "zh-CN" ? 1 : 0] : key
}

function days(count, language) {
    return language === "zh-CN" ? (count + " 天") : (count + " days")
}

function languageName(code, language) {
    if (!code) return ""
    var map = {
        auto: "auto", "zh-Hans": "chinese", "zh-Hant": "traditional",
        en: "english", ja: "japanese", ko: "korean",
        fr: "french", de: "german", ru: "russian", es: "spanish"
    }
    return map[code] ? text(map[code], language) : String(code)
}

function languages(language) {
    return [{value:"auto", label:text("auto",language)}, {value:"zh-Hans",label:text("chinese",language)},
        {value:"zh-Hant",label:text("traditional",language)}, {value:"en",label:text("english",language)},
        {value:"ja",label:text("japanese",language)}, {value:"ko",label:text("korean",language)},
        {value:"fr",label:text("french",language)}, {value:"de",label:text("german",language)},
        {value:"ru",label:text("russian",language)}, {value:"es",label:text("spanish",language)}]
}

function failure(code,language) {
    var messages={
        rate_limited:["Rate limited. Retry after the cooldown.","渠道限流，冷却结束后可重试。"],
        timeout:["The service timed out.","渠道响应超时。"], network:["Cannot connect to this service.","暂时无法连接该渠道。"],
        unauthorized:["Check the saved API key.","请检查已保存的 API 密钥。"], forbidden:["Check the account and service settings.","请检查账户与服务配置。"],
        unavailable:["This service is temporarily unavailable.","渠道暂时不可用。"], invalid_request:["Shorten the text or change the language pair.","请缩短文本或调整语向。"],
        invalid_response:["No usable translation was received.","没有收到可用译文。"], configuration:["Complete the service settings.","请补全服务配置。"],
        unsupported:["This language pair is not supported.","该渠道不支持此语向。"], cancelled:["Translation cancelled.","翻译已取消。"], internal:["The request failed. Retry.","请求未完成，请重试。"]
    }
    var message=messages[code]
    return message ? message[language==="zh-CN" ? 1 : 0] : text("unavailable",language)
}

function message(value,language) {
    if (!value) return ""
    var zh = language === "zh-CN"
    var idx = zh ? 1 : 0
    var known={
        // Status banners (preserve localization when toggling language)
        "Saved": labels.saved, "已保存": labels.saved,
        "Copied": labels.copied, "已复制": labels.copied,
        "Removed": labels.removed, "已移除": labels.removed,
        "Connected": labels.connected, "连接正常": labels.connected,
        "Translation service stopped. Retry to reconnect.": labels.backendFailed,
        "翻译服务已停止，重试以重新连接。": labels.backendFailed,
        "The operation timed out. Retry when the service responds.": labels.timeout,
        "操作超时，请待服务恢复后重试。": labels.timeout,
        "Too many pending operations. Wait and retry.": labels.overloaded,
        "待处理操作过多，请稍后重试。": labels.overloaded,
        "Translation interrupted; retry the unfinished services.": labels.interrupted,
        "翻译已中断，请重试未完成渠道。": labels.interrupted,
        // Startup & runtime
        "Missing backend executable. Run python3 scripts/install-omarchy-native.py to install the Rust backend.": [
            "Missing backend executable. Run python3 scripts/install-omarchy-native.py to install the Rust backend.",
            "未找到后端可执行文件，请运行 python3 scripts/install-omarchy-native.py 安装 Rust 后端。"
        ],
        "Too many pending operations": labels.overloaded,
        "Unsupported language": ["Unsupported language.", "不支持的语言。"],
        "Invalid parameters": ["Invalid request parameters.", "请求参数无效。"],
        "Command failed": ["The operation failed. Retry.", "操作未完成，请重试。"],
        // Config validation & keyring
        "Invalid service order": ["Invalid service order.", "服务顺序无效。"],
        "Invalid usage settings": ["Invalid usage settings.", "使用偏好无效。"],
        "Unsupported interface language": ["Unsupported interface language.", "不支持的界面语言。"],
        "API Key 过长": ["API key is too long.", "API 密钥过长。"],
        "启用前请填写 API Key": ["Enter an API key before enabling.", "启用前请填写 API 密钥。"],
        "请先保存 DeepL API Key": ["Save a DeepL API key first.", "请先保存 DeepL API 密钥。"],
        "请先在设置中保存 DeepL API Key": ["Save a DeepL API key in Settings first.", "请先在设置中保存 DeepL API 密钥。"],
        "启用 AI 前请填写 Base URL 和模型": ["Enter a Base URL and model before enabling AI.", "启用 AI 前请填写服务地址和模型。"],
        "请在设置中补全 AI 地址和模型": ["Enter the AI Base URL and model in Settings.", "请在设置中补全 AI 服务地址和模型。"],
        "Base URL 格式不正确": ["Invalid Base URL format.", "服务地址格式不正确。"],
        "云端地址必须使用 HTTPS，仅本机允许 HTTP；地址不能包含凭据或查询参数": [
            "Remote URLs must use HTTPS (HTTP is allowed for localhost only) without embedded credentials or query parameters.",
            "云端地址必须使用 HTTPS，仅本机允许 HTTP；地址不能包含凭据或查询参数。"
        ],
        "规则不能重复、源语言和目标语言不能相同，且必须使用支持的语言": [
            "Rules cannot repeat, source and target languages must differ, and all languages must be supported.",
            "规则不能重复、源语言和目标语言不能相同，且必须使用支持的语言。"
        ],
        "无法读取系统凭据存储，请在翻译服务设置中重新保存 API Key": [
            "Cannot read the system keyring. Re-save your API key in Settings.",
            "无法读取系统凭据存储，请在设置中重新保存 API 密钥。"
        ],
        "无法保存到系统凭据存储；原配置未更改。请解锁钥匙串/凭据服务后重试": [
            "Cannot save to the system keyring; settings unchanged. Unlock your keyring and retry.",
            "无法保存到系统凭据存储；原配置未更改。请解锁钥匙串后重试。"
        ],
        "配置文件无法解析；已保留原文件，请修复 config.json 后重启应用": [
            "Settings file could not be parsed; original preserved. Fix config.json and restart.",
            "配置文件无法解析；已保留原文件，请修复 config.json 后重启。"
        ],
        "无法读取配置文件，请检查目录权限后重试": [
            "Cannot read settings file. Check folder permissions and retry.",
            "无法读取配置文件，请检查目录权限后重试。"
        ],
        "配置目录无效": ["Cannot save settings; original configuration preserved.", "配置目录无效，原配置未更改。"],
        "无法创建配置目录": ["Cannot save settings; original configuration preserved.", "无法创建配置目录。"],
        "无法写入配置目录": ["Cannot save settings; original configuration preserved.", "无法写入配置目录。"],
        "配置序列化失败": ["Cannot save settings; original configuration preserved.", "配置序列化失败。"],
        "配置写入失败，原配置未更改": ["Cannot save settings; original configuration preserved.", "配置写入失败，原配置未更改。"],
        "配置替换失败，原配置未更改": ["Cannot save settings; original configuration preserved.", "配置替换失败，原配置未更改。"],
        "配置未初始化": ["Settings service is unavailable. Restart the plugin.", "配置未初始化，请重启插件。"],
        "配置服务不可用，请重启应用": ["Settings service is unavailable. Restart the plugin.", "配置服务不可用，请重启插件。"],
        "未知服务": ["Unknown translation service.", "未知翻译服务。"],
        // Translation & provider errors
        "请求编号无效": ["Invalid request ID.", "请求编号无效。"],
        "请输入 1–20000 个字符；较长内容请分段翻译": [
            "Enter 1–20,000 characters; split longer text into smaller parts.",
            "请输入 1–20000 个字符；较长内容请分段翻译。"
        ],
        "翻译队列不可用": ["Translation queue is unavailable.", "翻译队列不可用。"],
        "等待超时，可重试未完成的渠道": ["Timed out; retry unfinished services.", "等待超时，可重试未完成的渠道。"],
        "读取设置失败": ["Failed to load translation settings.", "读取翻译设置失败。"],
        "加载服务失败": ["Failed to load translation services.", "加载翻译服务失败。"],
        "没有启用的翻译服务，请在偏好设置中开启并完成配置": ["Enable a translation service in Settings.", "没有启用的翻译服务，请在设置中开启并完成配置。"],
        "翻译已完成，但历史记录保存失败": ["Translation completed; history could not be saved.", "翻译已完成，历史保存失败。"],
        "正在检测，请稍候": ["Connection test is already running. Please wait.", "正在测试连接，请稍候。"],
        "该渠道暂时限流，请稍后再试": [failure("rate_limited", "en"), failure("rate_limited", "zh-CN")],
        "渠道响应超时，已停止等待": [failure("timeout", "en"), failure("timeout", "zh-CN")],
        "暂时无法连接该渠道": [failure("network", "en"), failure("network", "zh-CN")],
        "服务凭据无效或已过期，请检查设置": [failure("unauthorized", "en"), failure("unauthorized", "zh-CN")],
        "服务拒绝访问，请检查账户或服务设置": [failure("forbidden", "en"), failure("forbidden", "zh-CN")],
        "渠道暂时不可用，请稍后再试": [failure("unavailable", "en"), failure("unavailable", "zh-CN")],
        "渠道未接受请求，可缩短文本或调整语向": [failure("invalid_request", "en"), failure("invalid_request", "zh-CN")],
        "暂时没有收到可用的译文": [failure("invalid_response", "en"), failure("invalid_response", "zh-CN")],
        "请检查翻译服务配置": [failure("configuration", "en"), failure("configuration", "zh-CN")],
        "该渠道暂不支持此语向": [failure("unsupported", "en"), failure("unsupported", "zh-CN")],
        "已取消等待": [failure("cancelled", "en"), failure("cancelled", "zh-CN")],
        "这次请求未能完成，请重试": [failure("internal", "en"), failure("internal", "zh-CN")],
        // Storage & diagnostics
        "存储尚未初始化": ["Local history storage is unavailable.", "本地历史存储尚未初始化。"],
        "存储服务不可用": ["Local history storage is unavailable.", "本地历史存储服务不可用。"],
        "数据库不可用": ["Local history database is unavailable.", "本地历史数据库不可用。"],
        "无法打开历史数据库，请检查目录权限或可用空间": [
            "Cannot open history database. Check disk space and folder permissions.",
            "无法打开历史数据库，请检查目录权限或可用空间。"
        ],
        "历史数据库初始化失败，原数据已保留": [
            "History database initialization failed; existing data preserved.",
            "历史数据库初始化失败，原数据已保留。"
        ],
        "数据操作失败，请检查磁盘空间和数据库权限后重试": [
            "Storage operation failed. Check disk space and permissions and retry.",
            "数据操作失败，请检查磁盘空间和数据库权限后重试。"
        ],
        "Search is too long": ["Search query is too long.", "搜索关键词过长。"],
        "日志目录不可用，请检查磁盘空间和目录权限": [
            "Log directory is unavailable. Check disk space and folder permissions.",
            "日志目录不可用，请检查磁盘空间和目录权限。"
        ],
        // Shortcuts (static messages from runtime/shortcuts.py)
        "快捷键格式无效": ["Invalid shortcut format.", "快捷键格式无效。"],
        "快捷键至少包含 Super、Ctrl 或 Alt": ["Shortcut must include at least Super, Ctrl, or Alt.", "快捷键至少包含 Super、Ctrl 或 Alt。"],
        "快捷键配置不完整": ["Incomplete shortcut configuration.", "快捷键配置不完整。"],
        "请以普通桌面用户运行，不要以 root 权限执行": ["Run as your desktop user, not root.", "请以普通桌面用户运行，不要以 root 权限执行。"],
        "需要 Omarchy 的 Hyprland Lua 配置": ["Requires Omarchy Hyprland Lua configuration.", "需要 Omarchy 的 Hyprland Lua 配置。"],
        "Native shortcut template unavailable": ["Shortcut template file is missing.", "缺少快捷键模板文件。"],
        // Desktop capture, OCR, annotation, speech & clipboard
        "A capture is already running": ["A capture is already running.", "当前已有截图或划词操作正在进行。"],
        "Select 1–20000 characters": ["Select 1–20,000 characters.", "请选择 1–20000 个字符。"],
        "Selection is not UTF-8 text": ["Selection is not valid UTF-8 text.", "选中内容不是有效的 UTF-8 文本。"],
        "No text was recognized": ["No text was recognized.", "未识别到文字。"],
        "Recognized text exceeds 20000 characters": ["Recognized text exceeds 20,000 characters.", "识别出的文字超过 20000 个字符。"],
        "Local OCR failed; check installed language data": [
            "Local OCR failed; check installed Tesseract language data.",
            "本地 OCR 识别失败，请检查已安装的 Tesseract 语言包。"
        ],
        "lucas-screenshot-editor is unavailable": [
            "Screenshot editor (lucas-screenshot-editor) is not installed.",
            "未找到截图标注编辑器（lucas-screenshot-editor）。"
        ],
        "Screenshot editor failed": ["Screenshot editor exited with an error.", "截图标注编辑器运行失败。"],
        "System screenshot capture failed": ["Screenshot capture failed. Check system screenshot tools.", "系统截图失败，请检查截图工具。"],
        "Screenshot capture failed": ["Screenshot capture failed. Check desktop screenshot tools.", "截图失败，请检查截图工具。"],
        "Pronunciation playback failed": ["Pronunciation playback failed. Check that mpv is installed.", "发音播放失败，请检查是否已安装 mpv。"],
        "Text is too large": ["Text is too large to copy.", "文本过长，无法复制。"],
        "Desktop operation timed out": ["Desktop operation timed out.", "桌面操作超时。"],
        "Required desktop tool is unavailable": ["A required desktop tool is unavailable.", "缺少必需的桌面工具。"]
    }
    if (known[value]) return known[value][idx]
    // Parametric shortcut validation and conflict messages from runtime/shortcuts.py.
    var dup = value.match(/^(.+) 与本应用的其他操作重复，请重新设置$/)
    if (dup) return zh ? value : (dup[1] + " is already used by another action. Choose a different shortcut.")
    var conflict = value.match(/^(.+) 与「(.+)」冲突，请重新设置$/)
    if (conflict) {
        if (zh) return value
        var target = conflict[2] === "已有桌面快捷键" ? "an existing desktop shortcut" : conflict[2]
        return conflict[1] + ' conflicts with "' + target + '". Choose a different shortcut.'
    }
    var unrecognized = value.match(/^无法识别快捷键 (.+)，示例：Super\+Ctrl\+Shift\+I$/)
    if (unrecognized) return zh ? value : ("Unrecognized shortcut " + unrecognized[1] + ". Example: Super+Ctrl+Shift+I.")
    var unsupportedKey = value.match(/^不支持按键 (.+)；请使用字母、数字、F1–F24 或导航键$/)
    if (unsupportedKey) return zh ? value : ("Unsupported key " + unsupportedKey[1] + ". Use a letter, number, F1–F24, or navigation key.")
    var manualEdit = value.match(/^(.+) 有手动修改；请先备份并移走该文件，再重新打开应用以启用设置页管理。原文件未覆盖。$/)
    if (manualEdit) return zh ? value : (manualEdit[1] + " has manual edits. Back up and move it aside, then reopen to manage shortcuts in Settings. Original file kept.")
    // Fallback category localization for other safe backend errors.
    if (!zh && /[\u3400-\u9fff]/.test(value)) return text("unavailable", language)
    if (zh && !/[\u3400-\u9fff]/.test(value)) {
        if (/clipboard|copy|wl-copy/i.test(value)) return "复制失败，请检查剪贴板服务。"
        if (/OCR|tesseract|recognized/i.test(value)) return "文字识别失败，请检查 OCR 语言数据。"
        if (/screenshot-editor|editor/i.test(value)) return "截图标注编辑器不可用。"
        if (/capture|screenshot|region/i.test(value)) return "截图失败，请检查截图工具。"
        if (/selection/i.test(value)) return "无法读取选区，请重试。"
        if (/speech|pronunciation|mpv/i.test(value)) return "发音播放失败，请检查是否已安装 mpv。"
        if (/shortcut/i.test(value)) return "快捷键操作失败。"
        return text("unavailable", language)
    }
    return value
}
