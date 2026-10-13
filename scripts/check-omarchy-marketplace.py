#!/usr/bin/env python3
"""Read-only eligibility check; never submits, pushes or edits desktop configuration."""
import datetime
import json
from pathlib import Path
import re
import subprocess
import urllib.parse

ROOT = Path(__file__).resolve().parents[1]


def command(*args):
    result = subprocess.run(args,cwd=ROOT,capture_output=True,text=True,timeout=30,check=True)
    return result.stdout.strip()


def api(path):
    return json.loads(command('gh','api',path))


def main():
    manifest = json.loads((ROOT / 'manifest.json').read_text())
    identity = manifest['id']
    remote = command('git','remote','get-url','origin')
    match = re.fullmatch(r'(?:git@github\.com:|https://github\.com/)([^/]+/[^/]+?)(?:\.git)?',remote)
    if not match:
        raise SystemExit('This checker requires a GitHub origin repository.')
    repository = match[1]
    metadata = api('repos/'+repository)
    head = api('repos/'+repository+'/commits/'+metadata['default_branch'])['sha']
    remote_names = [value['name'] for value in api('repos/'+repository+'/contents')]
    registry = json.loads(command('gh','api','repos/omacom/omarchy-plugin-marketplace/contents/registry.json',
                                  '--header','Accept: application/vnd.github.raw+json'))
    if not isinstance(registry,dict) or 'sources' not in registry or 'retiredPluginIds' not in registry:
        raise SystemExit('Marketplace registry response is incomplete; ID availability is unknown.')
    matches = []
    def inspect(value,path='$'):
        if isinstance(value,dict):
            for key,item in value.items():
                if key == identity: matches.append(path+'.'+key)
                inspect(item,path+'.'+key)
        elif isinstance(value,list):
            for index,item in enumerate(value): inspect(item,path+'['+str(index)+']')
        elif isinstance(value,str) and (value == identity or repository in value):
            matches.append(path)
    inspect(registry)
    query = urllib.parse.urlencode({'q':'repo:omacom/omarchy-plugin-marketplace "'+identity+'"'})
    issues = api('search/issues?'+query)['total_count']
    validation = subprocess.run(['omarchy','plugin','validate',str(ROOT)],capture_output=True,text=True,timeout=30)
    blockers = [
        {'id':'backend-setup','detail':'Shell cloning alone does not install the independent Rust executable; use the documented installer.'},
        {'id':'removal-and-update','detail':'Complete and verify removal/update instructions for backend, launcher, managed bindings and restored desktop integration.'},
    ]
    if 'manifest.json' not in remote_names:
        blockers.insert(0, {'id':'remote-source','detail':'The default branch does not contain the root plugin manifest.'})
    report = {
        'checked_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'eligible_for_submission_now':False,
        'repository':{'url':metadata['html_url'],'public':not metadata['private'],'archived':metadata['archived'],
                      'default_branch':metadata['default_branch'],'remote_sha':head,'local_sha':command('git','rev-parse','HEAD'),
                      'remote_root_files':remote_names,'native_code_pushed': 'manifest.json' in remote_names},
        'plugin':{'id':identity,'version':manifest['version'],'local_manifest':'manifest.json',
                  'local_validation_passed':validation.returncode==0,'root_manifest_exists':(ROOT / 'manifest.json').is_file(),
                  'native_readme_exists':(ROOT / 'README.md').is_file(),'native_license_exists':(ROOT / 'LICENSE').is_file(),
                  'registry_matches':matches,'historical_issue_matches':issues,
                  'id_available_at_check':not matches and not identity.startswith('omarchy.')},
        'blockers':blockers,
        'pending_validation':[
            'Unlocked interactive Wayland smoke test: focus, global shortcuts, region picker and real clipboard.',
            'Live providers and Secret Service credentials; GPU rendering and prolonged operation.',
            'Marketplace structure/compatibility and exact-commit Automated Security Baseline on the intended published repository.',
            'Owner confirmation of submission checklist and preview rights, followed by explicit approved-and-verified maintainer approval.'
        ],
        'recommendation':'Verify standalone installation, removal, update and preview documentation before requesting marketplace review.',
        'review_capabilities_to_expect':['installer','remote-build if source is built from the submitted repository','bundled-executable-binary if executable artifacts are committed'],
        'official_baseline_executed':False,'submitted':False,'listed':False,
        'suggested_category':'Productivity','suggested_tags':['quickshell','ai','hyprland'],
        'sources':['https://plugins.omarchy.org/publish.html',
                   'https://github.com/omacom/omarchy-plugin-marketplace/blob/main/SUBMISSION.md',
                   'https://github.com/omacom/omarchy-plugin-marketplace/blob/main/SECURITY.md',
                   'https://github.com/omacom/omarchy-plugin-marketplace/blob/main/VERIFICATION.md']
    }
    reports = ROOT / 'dist/omarchy/test-results'; reports.mkdir(parents=True,exist_ok=True)
    destination = reports / 'marketplace.json'
    destination.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n')
    print(json.dumps({'eligible':False,'local_validation_passed':validation.returncode==0,'registry_matches':matches,
                      'historical_issue_matches':issues,'report':str(destination)},indent=2))


if __name__ == '__main__':
    main()
