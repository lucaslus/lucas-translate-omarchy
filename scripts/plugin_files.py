"""Stage the same runtime layout from a source checkout or a release bundle."""
from pathlib import Path
import shutil
import sys

RUNTIME_FILES = ('backend.sh', 'dispatch.sh', 'launch.sh', 'shortcuts.py', 'shortcuts.lua')


def copy_plugin(source, destination):
    source, destination = Path(source), Path(destination)
    destination.mkdir(parents=True)
    for name in ('manifest.json', 'README.md', 'LICENSE'):
        shutil.copy2(source / name, destination / name)
    for folder, suffixes in [('qml', {'.qml', '.js'}), ('assets', {'.png'})]:
        for path in (source / folder).rglob('*'):
            if path.is_file() and path.suffix in suffixes:
                target = destination / path.relative_to(source)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(path, target)
    (destination / 'docs').mkdir()
    for name in ('MIGRATION.md', 'DEVELOPMENT.md'):
        shutil.copy2(source / 'docs' / name, destination / 'docs' / name)
    (destination / 'runtime').mkdir()
    for name in RUNTIME_FILES:
        shutil.copy2(source / 'runtime' / name, destination / 'runtime' / name)


if __name__ == '__main__':
    copy_plugin(Path(__file__).resolve().parents[1], sys.argv[1])
