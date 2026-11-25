#!/usr/bin/env python3
"""Exercise the archive as an installation and a separate library dependency."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def run(*args, cwd=ROOT, capture=False, env=None):
    return subprocess.run(args, cwd=cwd, check=True, text=True, env=env,
                          stdout=subprocess.PIPE if capture else None).stdout


metadata = json.loads(run('cargo', 'metadata', '--locked', '--no-deps',
                          '--format-version', '1', capture=True))
version = next(p['version'] for p in metadata['packages'] if p['name'] == 'anchor-book')
extra = ['--allow-dirty'] if sys.argv[1:] == ['--allow-dirty'] else []
if sys.argv[1:] and not extra:
    raise SystemExit('Usage: python3 scripts/check-package.py [--allow-dirty]')
run('cargo', 'package', '--locked', *extra)
package = Path(metadata['target_directory']) / 'package' / ('anchor-book-' + version)
for asset in ('contract/guidance.md', 'templates/project-guidance.md',
              'LICENSE-MIT', 'LICENSE-APACHE', 'build.rs', '.cargo_vcs_info.json'):
    assert (package / asset).is_file(), 'Missing packaged asset: ' + asset
run('cargo', '+1.88.0', 'build', '--locked', '--manifest-path', str(package / 'Cargo.toml'))
identity = json.loads((package / '.cargo_vcs_info.json').read_text())['git']['sha1']
assert identity == run('git', 'rev-parse', 'HEAD', capture=True).strip()
with tempfile.TemporaryDirectory(prefix='anchorbook-package-') as temporary:
    trial = Path(temporary)
    install = trial / 'installed'
    run('cargo', 'install', '--path', str(package), '--locked', '--root', str(install), cwd=trial)
    binary = str(install / 'bin' / 'anchorbook')
    reported = run(binary, '--version', cwd=trial, capture=True)
    assert version in reported and identity in reported, reported
    contract = run(binary, 'contract', cwd=trial, capture=True)
    assert contract == run('cargo', 'run', '--quiet', '--locked', '--bin',
                           'anchorbook', '--', 'contract', capture=True)
    template = run(binary, 'template', 'project-guidance', cwd=trial, capture=True)
    assert template.strip() == (package / 'templates/project-guidance.md').read_text().strip()
    docs = trial / 'docs'
    docs.mkdir()
    (docs / 'SUMMARY.md').write_text('# Summary\n\n- [Design](design.md)\n- [Authority](authority.md)\n- [Work](roadmap.md)\n')
    for name in ('design', 'authority', 'roadmap'):
        (docs / (name + '.md')).write_text('# ' + name.title() + '\n\nPackage acceptance.\n')
    (trial / 'anchorbook.toml').write_text('''schema-version = 1
[project]
phase = "prototyping"
[book]
source = "docs"
output = "generated/site"
mdbook = "book.toml"
[book.sections]
design = "design.md"
authority = "authority.md"
implementation-roadmaps = "roadmap.md"
''')
    (trial / 'book.toml').write_text('''[book]
src = "docs"
[build]
build-dir = "generated/site"
[preprocessor.anchorbook]
command = "anchorbook md"
config = "anchorbook.toml"
''')
    run(binary, 'check', '--config', 'anchorbook.toml', cwd=trial)
    environment = dict(os.environ, PATH=str(install / 'bin') + os.pathsep + os.environ['PATH'])
    run('mdbook', 'build', cwd=trial, env=environment)
    assert 'Package acceptance.' in (trial / 'generated/site/design.html').read_text()
    consumer = trial / 'consumer'
    (consumer / 'src').mkdir(parents=True)
    manifest = ('[package]\nname = "package-consumer"\nversion = "0.0.0"\nedition = "2024"\n'
                '[dependencies]\nanchor-book = { path = ' + json.dumps(str(package)) + ' }\n')
    (consumer / 'Cargo.toml').write_text(manifest)
    (consumer / 'src/main.rs').write_text('''use anchor_book::{Chapter, read_records};
use std::path::Path;
fn main() {
    let chapters = [Chapter::new(Path::new("design.md"), "# Design")];
    assert!(read_records(&chapters).unwrap().is_empty());
}
''')
    graph = json.loads(run('cargo', 'metadata', '--format-version', '1', cwd=consumer, capture=True))
    selected = {n['id'] for n in graph['resolve']['nodes']}
    assert all(p['name'] != 'vstd' for p in graph['packages'] if p['id'] in selected)
    run('cargo', 'run', '--locked', cwd=consumer)
    # The consumer may independently select a verifier library different from
    # the engine's optional pin, without enabling the engine's proof feature.
    (consumer / 'Cargo.toml').write_text(manifest + 'vstd = "=0.0.0-2026-11-22-0344"\n')
    run('cargo', 'check', cwd=consumer)
print('Packaged CLI, contract, template, Book render, and library consumers passed.')
