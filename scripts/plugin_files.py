"""Stage runtime plugin files from the standalone source or a release bundle."""
from pathlib import Path
import shutil
import sys

RUNTIME_SCRIPTS = ('backend.sh', 'dispatch.sh', 'launch.sh', 'shortcuts.py')


def copy_plugin(source, destination):
    source, destination = Path(source), Path(destination)
    destination.mkdir(parents=True)
    for name in ('manifest.json', 'README.md', 'LICENSE', 'shortcuts.lua'):
        shutil.copy2(source / name, destination / name)
    for pattern in ('*.qml', '*.js'):
        for path in source.glob(pattern):
            shutil.copy2(path, destination / path.name)
    shutil.copytree(source / 'logos', destination / 'logos')
    (destination / 'docs').mkdir()
    shutil.copy2(source / 'docs/MIGRATION.md', destination / 'docs/MIGRATION.md')
    (destination / 'scripts').mkdir()
    for name in RUNTIME_SCRIPTS:
        shutil.copy2(source / 'scripts' / name, destination / 'scripts' / name)


if __name__ == '__main__':
    copy_plugin(Path(__file__).resolve().parents[1], sys.argv[1])
