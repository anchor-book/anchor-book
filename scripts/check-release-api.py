#!/usr/bin/env python3
"""Check the ordinary library API against crates.io; never enable Verus."""
import json
import subprocess
import urllib.error
import urllib.request

request = urllib.request.Request('https://crates.io/api/v1/crates/anchor-book',
                                 headers={'User-Agent': 'anchor-book release checks'})
try:
    with urllib.request.urlopen(request, timeout=30) as response:
        baseline = json.load(response)['crate']['max_stable_version']
except urllib.error.HTTPError as error:
    if error.code != 404:
        raise
    baseline = None
if baseline:
    subprocess.run(['cargo', 'semver-checks', 'check-release', '--default-features',
                    '--baseline-version', baseline], check=True)
else:
    print('No published stable baseline: API comparison starts after the first release.')
