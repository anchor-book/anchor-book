# Project Integration

**Roadmap identifier:** `project-integration`

## Goal

Two existing Rust projects, Symposium and then sage-dev, develop with their own
Anchor Books: agents start from the contract and the project's Book, the project's
ordinary commands check and render it, and the gaps found become engine work here.
Each project keeps its existing user Book and adds a separate Anchor Book, adapted
from its design notes and decision records. Provisional anchor conventions in
either project are dropped and replaced with Anchor Book records, not migrated.

[Project Integration](../design/project-integration.md) defines the goal design.
This roadmap is the trial that exercises it; it does not replace the release
work in [later work](index.md#later-work-retained-from-the-design).

## Sequence and scope

Milestone 1 integrates Symposium and builds the shared groundwork: locating
`book.toml`, running the engine from another project, giving its agents the
contract, and rendering with the project's mdBook. Milestone 2 integrates sage-dev
on that groundwork and adds what its setup needs: a pinned nightly toolchain and
other mdBook preprocessors. Both are prototyping work on branches of the projects'
forks; changes to the projects themselves are their maintainers' decisions.

Formal verification in these projects is outside this roadmap until a project
needs it; it depends on [engine proof isolation](formal-verification.md#formal-verification-4)
and the [Verification Evidence roadmap](verification-evidence.md).

<a id="project-integration-1"></a>

## Milestone 1. Symposium develops with its own Anchor Book

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">On a Symposium branch, a separate Anchor Book beside the existing user Book passes the structural check and renders through Symposium's own commands, and a fresh agent session starts from the contract and that Book. Each gap found is fixed in the engine or recorded as engine work.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">The structural checker and record renderer. Agreement on this milestone's scope.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/project-integration.html">Project Integration</a>, <a href="../design/configuration.html#relationship-to-mdbook">mdBook configuration location</a>, and <a href="../design/system-overview.html#contract-delivery">contract delivery</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>project-integration-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> <strong>Configuration:</strong> Implement <code>[book].mdbook</code>, resolving mdBook's source and output relative to that file, and migrate this repository's configuration to name its <code>book.toml</code>.</label></li>
<li><label><input type="checkbox" disabled> <strong>Running the engine:</strong> Choose how Symposium runs a pinned engine revision before crate release, such as installing from a Git revision or a workspace binary, and document it.</label></li>
<li><label><input type="checkbox" disabled> <strong>Contract:</strong> Choose how Symposium's agents read the contract before <code>anchorbook contract</code> exists, without keeping an edited copy.</label></li>
<li><label><input type="checkbox" disabled> <strong>mdBook version:</strong> Symposium installs the newest mdBook; either pin 0.5.4 for the Anchor Book or add support for the version it uses, with tests.</label></li>
<li><label><input type="checkbox" disabled> <strong>The Book:</strong> Create the Anchor Book with its configuration, navigation, project guidance chapter, Authority adapted from Symposium's RFDs, one design area with anchors adapted from its design notes, a roadmap, and the <code>AGENTS.md</code> bootstrap. Drop the provisional anchor markup.</label></li>
<li><label><input type="checkbox" disabled> <strong>Use it:</strong> Run the check and render from Symposium, and exercise a fresh agent session that reads the contract, guidance, and Book and identifies the phase and commands.</label></li>
<li><label><input type="checkbox" disabled> <strong>Record:</strong> List every gap found, fix those in scope, and add the rest as engine work in the relevant roadmap.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="project-integration-1-mdbook-location" class="roadmap-criterion-summary"><th scope="row">mdBook location</th><td><p class="roadmap-condition">The loader accepts a configured <code>book.toml</code> in a subdirectory and compares its resolved source and output; fixtures cover a subdirectory, a mismatch, a missing file, and an escaping path. This repository's configuration names its <code>book.toml</code> and still passes.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for mdBook location" aria-expanded="true" aria-controls="project-integration-1-mdbook-location-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-mdbook-location-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>mdbook-location</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-renderer-alignment">check-renderer-alignment</a> · <code>resolved-comparison</code>, <code>renderer-input</code> — Partial: a configured <code>book.toml</code> location; <a href="../design/configuration.html#check-configuration-validity">check-configuration-validity</a> · <code>path-boundaries</code> — Partial: the new field.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-1-native-commands" class="roadmap-criterion-summary"><th scope="row">Native commands</th><td><p class="roadmap-condition">On the Symposium branch, the documented commands check and render the Anchor Book with a pinned engine revision, leave the user Book unchanged, and refuse to render a deliberately broken copy.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for native commands" aria-expanded="true" aria-controls="project-integration-1-native-commands-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-native-commands-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>native-commands</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-native-commands">project-native-commands</a> · <code>consumer-fixture</code> — Partial: one real project with a pinned revision rather than a packaged crate; <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>renderer-versions</code> — Partial: the mdBook version Symposium uses.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-1-agent-start" class="roadmap-criterion-summary"><th scope="row">Agent start</th><td><p class="roadmap-condition">A fresh agent session in the Symposium branch reads the contract, project guidance, and Book through the bootstrap, and identifies the phase, commands, and governing records of the adapted design area. Record agent versions and any assistance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent start" aria-expanded="true" aria-controls="project-integration-1-agent-start-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-agent-start-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-start</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-agent-bootstrap">project-agent-bootstrap</a> · <code>fresh-session</code> — Partial: interim contract access; <code>contract-retrieval</code> needs the contract command.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-1-gaps" class="roadmap-criterion-summary"><th scope="row">Gaps recorded</th><td><p class="roadmap-condition">Every gap found has a fix with evidence or an entry in the relevant roadmap; the Anchor Book's adapted content is reviewed with the engineer.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for gaps recorded" aria-expanded="true" aria-controls="project-integration-1-gaps-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-gaps-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>gaps</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — trial record.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="project-integration-2"></a>

## Milestone 2. sage-dev develops with its own Anchor Book

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">On a sage-dev branch, a separate Anchor Book replaces the provisional anchor markup, passes the structural check, and renders through sage-dev's commands on its pinned nightly toolchain alongside its other mdBook preprocessors; a fresh agent session starts from it.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="#project-integration-1">Milestone 1</a>, whose groundwork it reuses.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/project-integration.html">Project Integration</a>, including <a href="../design/project-integration.html#what-the-crate-must-not-impose">what the crate must not impose</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>project-integration-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Build and run the pinned engine revision with sage-dev's pinned nightly toolchain; fix or record any incompatibility.</label></li>
<li><label><input type="checkbox" disabled> Create the Anchor Book from sage-dev's design material, as in milestone 1, and replace its provisional anchors with Anchor Book records. Removing the provisional tooling is the maintainers' decision.</label></li>
<li><label><input type="checkbox" disabled> Render with the Anchor Book preprocessor in the Anchor Book's own <code>book.toml</code>; confirm the user Book and its preprocessors are unaffected.</label></li>
<li><label><input type="checkbox" disabled> Exercise a fresh agent session and record gaps as in milestone 1.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="project-integration-2-nightly-toolchain" class="roadmap-criterion-summary"><th scope="row">Nightly toolchain</th><td><p class="roadmap-condition">The engine builds and checks the Anchor Book on sage-dev's pinned nightly toolchain without changing sage-dev's toolchain or dependencies.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for nightly toolchain" aria-expanded="true" aria-controls="project-integration-2-nightly-toolchain-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-2-nightly-toolchain-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>nightly-toolchain</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>dependency-review</code> — Partial: one pinned nightly toolchain.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-2-native-commands" class="roadmap-criterion-summary"><th scope="row">Native commands</th><td><p class="roadmap-condition">sage-dev's documented commands check and render the Anchor Book, its user Book and preprocessors are unaffected, and a deliberately broken copy is not rendered.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for native commands" aria-expanded="true" aria-controls="project-integration-2-native-commands-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-2-native-commands-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>native-commands</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-native-commands">project-native-commands</a> · <code>consumer-fixture</code> — Partial: a second project with other preprocessors.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-2-agent-start" class="roadmap-criterion-summary"><th scope="row">Agent start and gaps</th><td><p class="roadmap-condition">A fresh agent session starts from the contract and the Anchor Book, and every gap found is fixed or recorded, as in milestone 1.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent start and gaps" aria-expanded="true" aria-controls="project-integration-2-agent-start-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-2-agent-start-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-start</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-agent-bootstrap">project-agent-bootstrap</a> · <code>fresh-session</code> — Partial: second project.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>
