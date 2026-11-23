#!/usr/bin/env python3
"""Install only the native Omarchy variant; preserve the Tauri source and data."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

from plugin_files import copy_plugin

ROOT = Path(__file__).resolve().parents[1]
PLUGIN_ID = 'lucas.translate'
DESKTOP_ICON = Path('icons/hicolor/256x256/apps/lucas-translate.png')
HYPR_INCLUDE = 'dofile(os.getenv("HOME") .. "/.config/hypr/lucas-translate.lua")'
HYPR_COMMENT = '-- Lucas Translate floating panel and shortcuts'


def run(*arguments, **options):
    return subprocess.run(arguments, check=True, text=True, **options)


def require_unlocked():
    result = subprocess.run(['omarchy-shell', 'lock', 'isLocked'], capture_output=True, text=True)
    if result.returncode == 0 and result.stdout.strip() == 'true':
        raise RuntimeError('Unlock the desktop before installing or updating a plugin; this Shell version can crash on a plugin reload while locked.')
    if result.returncode != 0 or result.stdout.strip() != 'false':
        raise RuntimeError('Cannot confirm the desktop is unlocked; no plugin files were modified.')
    monitors = run('hyprctl', '-j', 'monitors', capture_output=True)
    if not any(not monitor.get('disabled', False) for monitor in json.loads(monitors.stdout)):
        raise RuntimeError('No active compositor monitor; plugin reload refused.')


def stamp_revision(staging, binary):
    digest = hashlib.sha256()
    for path in sorted(staging.rglob('*')):
        if path.is_file() and path.name != 'BuildInfo.js':
            digest.update(str(path.relative_to(staging)).encode() + b'\0')
            digest.update(path.read_bytes() + b'\0')
    digest.update(binary.read_bytes())
    revision = digest.hexdigest()
    (staging / 'qml/BuildInfo.js').write_text('var revision = ' + json.dumps(revision) + '\n')
    return revision


def wait_ready():
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        health = run('omarchy-shell', 'shell', 'call', PLUGIN_ID, 'health', '', capture_output=True).stdout.strip()
        if health == 'ready':
            return
        time.sleep(0.2)
    raise RuntimeError('Native panel or backend did not become ready; installation rolled back.')


def verify_revision(expected):
    wait_ready()
    def loaded():
        return run('omarchy-shell', 'shell', 'call', PLUGIN_ID, 'health', 'revision', capture_output=True).stdout.strip()
    if loaded() == expected:
        return
    # This Shell can retain QML/JS caches after rescanPlugins and enable/disable.
    require_unlocked()
    run('omarchy', 'restart', 'shell', capture_output=True)
    wait_ready()
    if loaded() != expected:
        raise RuntimeError('Shell is still running an outdated native UI; installation rolled back.')


def backup(path, destination, *, follow_symlinks=True):
    if not follow_symlinks and path.is_symlink():
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.symlink_to(path.readlink())
        return
    if not path.exists():
        return
    destination.parent.mkdir(parents=True, exist_ok=True)
    if path.is_dir():
        shutil.copytree(path, destination)
    else:
        shutil.copy2(path, destination)


def remove_managed_hypr_include(hyprland_lua):
    """Remove only this plugin's include line from live hyprland.lua, preserving user edits."""
    if not hyprland_lua.is_file():
        return
    original = hyprland_lua.read_text(encoding='utf-8', errors='surrogateescape')
    lines = original.splitlines(keepends=True)
    kept = []
    removed = False
    for line in lines:
        if line.strip() == HYPR_INCLUDE:
            removed = True
            if kept and kept[-1].strip() == HYPR_COMMENT:
                kept.pop()
                if kept and kept[-1].strip() == '':
                    kept.pop()
            continue
        kept.append(line)
    if not removed:
        return
    temp = hyprland_lua.with_name(hyprland_lua.name + '.lucas-rollback-tmp')
    temp.write_text(''.join(kept), encoding='utf-8', errors='surrogateescape')
    temp.replace(hyprland_lua)


def current_plugin_enabled():
    try:
        discovered = json.loads(run('omarchy-shell', 'shell', 'listPlugins', capture_output=True).stdout)
    except Exception:
        return None
    for item in discovered:
        if isinstance(item, dict) and item.get('id') == PLUGIN_ID:
            return bool(item.get('enabled'))
    return None


def restore_file(path, previous):
    if path.is_symlink():
        path.unlink()
    elif path.is_dir():
        shutil.rmtree(path)
    elif path.exists():
        path.unlink()
    if previous.is_symlink():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.symlink_to(previous.readlink())
    elif previous.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        if previous.is_dir():
            shutil.copytree(previous, path)
        else:
            shutil.copy2(previous, path)


def restore(owned_paths, shortcut_paths, saved, *, without_shortcuts, touched, previous_enabled, hyprland_lua, had_hypr_include):
    require_unlocked()
    # Roll back only files owned by this plugin; never overwrite shared shell.json
    # or hyprland.lua from old backups, which could discard independent user edits.
    if touched['plugin']:
        for path, name in owned_paths:
            restore_file(path, saved / name)
    if not without_shortcuts and touched['shortcuts']:
        for path, name in shortcut_paths:
            restore_file(path, saved / name)
        if not had_hypr_include:
            remove_managed_hypr_include(hyprland_lua)
        run('hyprctl', 'reload', capture_output=True)
    if touched['shell']:
        run('omarchy-shell', 'shell', 'rescanPlugins', capture_output=True)
        run('omarchy-shell', 'shell', 'setPluginEnabled', PLUGIN_ID, 'true' if previous_enabled else 'false', capture_output=True)
        run('omarchy-shell', 'shell', 'reloadConfig', capture_output=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, help='Use an already built backend')
    parser.add_argument('--without-shortcuts', action='store_true')
    args = parser.parse_args()
    if os.geteuid() == 0:
        raise SystemExit('Run as the desktop user, without sudo.')
    shell = Path(os.environ.get('OMARCHY_PATH', '/usr/share/omarchy')) / 'shell'
    if not (shell / 'Ui/BorderSurface.qml').is_file():
        raise SystemExit('This variant requires the Omarchy Quickshell plugin runtime.')
    run('omarchy-shell', 'shell', 'ping', capture_output=True)
    require_unlocked()
    if args.binary:
        binary = args.binary.resolve()
    else:
        run('cargo', 'build', '--release', '--locked', '--manifest-path', str(ROOT / 'Cargo.toml'), '--package', 'lucas-translate-omarchy-backend')
        binary = ROOT / 'target/release/lucas-translate-omarchy-backend'
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise SystemExit(f'Missing backend executable: {binary}')
    config = Path(os.environ.get('XDG_CONFIG_HOME', str(Path.home() / '.config')))
    data = Path(os.environ.get('XDG_DATA_HOME', ''))
    if not data.is_absolute():
        data = Path.home() / '.local/share'
    state = Path(os.environ.get('XDG_STATE_HOME', str(Path.home() / '.local/state')))
    plugins = config / 'omarchy/plugins'
    target = plugins / PLUGIN_ID
    plugins.mkdir(parents=True, exist_ok=True)
    saved = state / 'lucas-translate-omarchy/install-backups' / str(time.time_ns())
    destination = Path.home() / '.local/bin'
    destination.mkdir(parents=True, exist_ok=True)
    desktop = data / 'applications/lucas-translate.desktop'
    icon = data / DESKTOP_ICON
    hyprland_lua = config / 'hypr/hyprland.lua'
    previous_enabled = current_plugin_enabled()
    had_hypr_include = (
        not args.without_shortcuts
        and hyprland_lua.is_file()
        and HYPR_INCLUDE.encode() in hyprland_lua.read_bytes()
    )
    # Keep backups of plugin-owned files (plus read-only snapshots of shared configs
    # for manual reference; shared files are never overwritten on automatic rollback).
    owned_paths = [
        (target, 'plugin'),
        (destination / 'lucas-translate-omarchy-backend', 'backend'),
        (destination / 'lucas-translate-native', 'launcher'),
        (desktop, 'desktop'),
        (icon, 'icon'),
        (destination / 'lucas-translate', 'dispatch'),
    ]
    shortcut_paths = [
        (config / 'hypr/lucas-translate.lua', 'lucas-translate.lua'),
        (config / 'hypr/lucas-translate-shortcuts.json', 'shortcuts.json'),
    ]
    for path, name in owned_paths:
        backup(path, saved / name, follow_symlinks=path != icon)
    if not args.without_shortcuts:
        for path, name in shortcut_paths:
            backup(path, saved / name)
        backup(hyprland_lua, saved / 'hyprland.lua')
    touched = {'plugin': False, 'shell': False, 'shortcuts': False}
    try:
        install(binary, config, target, destination, desktop, icon, args.without_shortcuts, touched)
    except Exception:
        restore(
            owned_paths,
            shortcut_paths,
            saved,
            without_shortcuts=args.without_shortcuts,
            touched=touched,
            previous_enabled=previous_enabled,
            hyprland_lua=hyprland_lua,
            had_hypr_include=had_hypr_include,
        )
        raise
    print(f'Native Omarchy plugin installed. Previous files: {saved}')


def install(binary, config, target, destination, desktop, icon, without_shortcuts, touched):
    require_unlocked()
    with tempfile.TemporaryDirectory(prefix='lucas-native-install-', dir=target.parent.parent) as directory:
        staging = Path(directory) / PLUGIN_ID
        copy_plugin(ROOT, staging)
        revision = stamp_revision(staging, binary)
        run('omarchy', 'plugin', 'validate', str(staging))
        require_unlocked()
        touched['shell'] = True
        touched['plugin'] = True
        run('omarchy-shell', 'shell', 'setPluginEnabled', PLUGIN_ID, 'false', capture_output=True)
        if target.exists():
            shutil.rmtree(target)
        staging.replace(target)
    # Atomic executable replacement works even while the old inode is running.
    for source, name in [(binary, 'lucas-translate-omarchy-backend'),
                         (ROOT / 'runtime/launch.sh', 'lucas-translate-native'),
                         (ROOT / 'runtime/dispatch.sh', 'lucas-translate')]:
        temp = destination / (name + '.installing')
        shutil.copy2(source, temp)
        temp.chmod(0o755)
        temp.replace(destination / name)
    icon.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='lucas-icon-', dir=icon.parent) as directory:
        staged_icon = Path(directory) / icon.name
        shutil.copy2(ROOT / 'assets' / icon.name, staged_icon)
        staged_icon.replace(icon)
    icon_value = icon.stem
    if icon != Path.home() / '.local/share' / DESKTOP_ICON:
        # Omarchy's live icon index does not scan a custom XDG_DATA_HOME.
        icon_value = str(icon).replace('\\', '\\\\').replace('\n', '\\n').replace('\r', '\\r').replace('\t', '\\t')
    desktop.parent.mkdir(parents=True, exist_ok=True)
    desktop.write_text('[Desktop Entry]\nType=Application\nName=Lucas Translate\nComment=Native Omarchy translation and local OCR\n'
                       f'Exec={destination / "lucas-translate"} --show\nIcon={icon_value}\nTerminal=false\nCategories=Utility;\n')
    require_unlocked()
    run('omarchy-shell', 'shell', 'rescanPlugins', capture_output=True)
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        discovered = json.loads(run('omarchy-shell', 'shell', 'listPlugins', capture_output=True).stdout)
        if any(item.get('id') == PLUGIN_ID for item in discovered):
            break
        time.sleep(0.2)
    else:
        raise RuntimeError('Shell did not discover the native plugin; installation rolled back.')
    enabled = run('omarchy-shell', 'shell', 'setPluginEnabled', PLUGIN_ID, 'true', capture_output=True).stdout.strip()
    if enabled != 'ok':
        raise RuntimeError('Shell refused to enable the native plugin; installation rolled back.')
    verify_revision(revision)
    if not without_shortcuts:
        touched['shortcuts'] = True
        run('python3', str(target / 'runtime/shortcuts.py'))
        run('hyprctl', 'reload', capture_output=True)
        errors = run('hyprctl', 'configerrors', capture_output=True).stdout.strip()
        if errors:
            raise RuntimeError(errors)
    installed = json.loads(run('omarchy-shell', 'shell', 'listPlugins', capture_output=True).stdout)
    if not any(item.get('id') == PLUGIN_ID and item.get('enabled') for item in installed):
        raise RuntimeError('Plugin was not enabled; installation rolled back.')


if __name__ == '__main__':
    main()
