# Project Integration

**Roadmap identifier:** `project-integration`

## Goal

This Book first, then two existing Rust projects, Symposium and sage-dev, develop
with Anchor Books: agents start from the contract and the project's Book, the project's
ordinary commands check and render it, and the gaps found become engine work here.
Each project keeps its existing user Book and adds a separate Anchor Book, adapted
from its design notes and decision records. Provisional anchor conventions in
either project are dropped and replaced with Anchor Book records, not migrated.

[Project Integration](../design/project-integration.md) defines the goal design.
This roadmap is the trial that exercises it; it does not replace the release
work in [later work](index.md#later-work-retained-from-the-design).

## Sequence and scope

**This Book first.** Every capability a project relies on is implemented and in
use in this Book before a project adopts it, so this Book stays the primary
acceptance case. Milestone 1 does that for the shared groundwork: locating
`book.toml`, retrieving the contract, running a pinned engine revision, and the
supported mdBook versions. Milestone 2 then integrates Symposium, and milestone 3
sage-dev, adding its pinned nightly toolchain and other mdBook preprocessors. A gap
found in a project is fixed and exercised here before the project relies on the
fix. Project work happens on branches of the projects' forks; changes to the
projects themselves are their maintainers' decisions.

Formal verification in these projects is outside this roadmap until a project
needs it; it depends on [engine proof isolation](formal-verification.md#formal-verification-4)
and the [Verification Evidence roadmap](verification-evidence.md).

<a id="project-integration-1"></a>

## Milestone 1. This Book uses the integration groundwork

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">This repository uses every mechanism a project will rely on: its configuration names its <code>book.toml</code>, its agents retrieve the contract with <code>anchorbook contract</code>, a pinned engine revision installed the documented way checks and renders this Book, and the declared mdBook versions are tested.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1">The configuration loader, structural checker, and record renderer. Agreement on this milestone's scope and on moving the contract source into the engine.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/configuration.html#relationship-to-mdbook">mdBook configuration location</a>, <a href="../design/system-overview.html#contract-delivery">contract delivery</a>, <a href="../design/project-integration.html#running-the-engine">running the engine</a>, and <a href="../design/project-integration.html#agent-bootstrap">agent bootstrap</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>project-integration-1</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> <strong>Configuration:</strong> Implement <code>[book].mdbook</code>, resolving mdBook's source and output relative to that file, and migrate this repository's configuration to name its <code>book.toml</code>.</label></li>
<li><label><input type="checkbox" disabled> <strong>Contract:</strong> Move the maintained contract source into the engine, add <code>anchorbook contract</code>, switch this repository's <code>AGENTS.md</code> to retrieve it, and remove the temporary Book copy and its navigation entry, as <a href="../design/system-overview.html#contract-delivery">contract delivery</a> describes.</label></li>
<li><label><input type="checkbox" disabled> <strong>Running the engine:</strong> Choose and document how a project installs a pinned engine revision before crate release, and exercise that path in this repository's CI against this Book.</label></li>
<li><label><input type="checkbox" disabled> <strong>mdBook versions:</strong> Decide which mdBook versions the preprocessor supports, including the version Symposium uses or a documented pin, and test each against this Book.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="project-integration-1-mdbook-location" class="roadmap-criterion-summary"><th scope="row">mdBook location</th><td><p class="roadmap-condition">The loader accepts a configured <code>book.toml</code>, including one in a subdirectory, and compares its resolved source and output; fixtures cover a subdirectory, a mismatch, a missing file, and an escaping path. This repository's configuration names its <code>book.toml</code> and its Book still passes and renders.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for mdBook location" aria-expanded="true" aria-controls="project-integration-1-mdbook-location-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-mdbook-location-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>mdbook-location</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/structural-checking.html#check-renderer-alignment">check-renderer-alignment</a> · <code>resolved-comparison</code>, <code>renderer-input</code> — Partial: a configured <code>book.toml</code> location; <a href="../design/configuration.html#check-configuration-validity">check-configuration-validity</a> · <code>path-boundaries</code> — Partial: the new field.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-1-contract" class="roadmap-criterion-summary"><th scope="row">Contract</th><td><p class="roadmap-condition"><code>anchorbook contract</code> prints the contract packaged with the running engine without reading project files; this repository's agents start from it, and a fresh session in this repository reads it, the project guidance, and the Book. No copy of the contract remains in the Book.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for contract" aria-expanded="true" aria-controls="project-integration-1-contract-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-contract-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>contract</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-agent-bootstrap">project-agent-bootstrap</a> · <code>contract-retrieval</code> — Full; <code>fresh-session</code> — Partial: this repository.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-1-installed-engine" class="roadmap-criterion-summary"><th scope="row">Installed engine</th><td><p class="roadmap-condition">A CI job installs the pinned engine revision the documented way and checks and renders this Book with it, with the same findings as the in-repository build; each declared mdBook version renders this Book, and an undeclared version is rejected with a diagnostic naming the supported ones.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for installed engine" aria-expanded="true" aria-controls="project-integration-1-installed-engine-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-1-installed-engine-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>installed-engine</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>renderer-versions</code> — Full for the declared versions; <a href="../design/project-integration.html#project-one-engine-version">project-one-engine-version</a> · <code>same-version-entry-points</code> — Partial: installed and in-repository builds of one revision.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="project-integration-2"></a>

## Milestone 2. Symposium develops with its own Anchor Book

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">On a Symposium branch, a separate Anchor Book beside the existing user Book passes the structural check and renders through Symposium's own commands, and a fresh agent session starts from the contract and that Book. Each gap found is fixed in the engine or recorded as engine work.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="#project-integration-1">Milestone 1</a> and agreement on this milestone's scope.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/project-integration.html">Project Integration</a>, <a href="../design/configuration.html#relationship-to-mdbook">mdBook configuration location</a>, and <a href="../design/system-overview.html#contract-delivery">contract delivery</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>project-integration-2</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> <strong>Setup:</strong> Install the engine at the pinned revision and configure rendering, as documented by milestone 1.</label></li>
<li><label><input type="checkbox" disabled> <strong>The Book:</strong> Create the Anchor Book with its configuration, navigation, project guidance chapter, Authority adapted from Symposium's RFDs, one design area with anchors adapted from its design notes, a roadmap, and the <code>AGENTS.md</code> bootstrap. Drop the provisional anchor markup.</label></li>
<li><label><input type="checkbox" disabled> <strong>Use it:</strong> Run the check and render from Symposium, and exercise a fresh agent session that reads the contract, guidance, and Book and identifies the phase and commands.</label></li>
<li><label><input type="checkbox" disabled> <strong>Record:</strong> List every gap found. Fix each one in the engine and exercise it in this Book before Symposium relies on it, or add it as engine work in the relevant roadmap.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="project-integration-2-native-commands" class="roadmap-criterion-summary"><th scope="row">Native commands</th><td><p class="roadmap-condition">On the Symposium branch, the documented commands check and render the Anchor Book with a pinned engine revision, leave the user Book unchanged, and refuse to render a deliberately broken copy.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for native commands" aria-expanded="true" aria-controls="project-integration-2-native-commands-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-2-native-commands-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>native-commands</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-native-commands">project-native-commands</a> · <code>consumer-fixture</code> — Partial: one real project with a pinned revision rather than a packaged crate; <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>renderer-versions</code> — Partial: the mdBook version Symposium uses.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-2-agent-start" class="roadmap-criterion-summary"><th scope="row">Agent start</th><td><p class="roadmap-condition">A fresh agent session in the Symposium branch reads the contract, project guidance, and Book through the bootstrap, and identifies the phase, commands, and governing records of the adapted design area. Record agent versions and any assistance.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent start" aria-expanded="true" aria-controls="project-integration-2-agent-start-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-2-agent-start-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-start</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-agent-bootstrap">project-agent-bootstrap</a> · <code>fresh-session</code> — Partial: a second project, using the contract command from milestone 1.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-2-gaps" class="roadmap-criterion-summary"><th scope="row">Gaps recorded</th><td><p class="roadmap-condition">Every gap found has a fix with evidence or an entry in the relevant roadmap; the Anchor Book's adapted content is reviewed with the engineer.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for gaps recorded" aria-expanded="true" aria-controls="project-integration-2-gaps-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-2-gaps-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>gaps</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> None — trial record.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>

<a id="project-integration-3"></a>

## Milestone 3. sage-dev develops with its own Anchor Book

<div class="roadmap-table-wrap">
<table class="roadmap-criteria roadmap-milestone">
<colgroup><col style="width:25%"><col style="width:75%"></colgroup>
<tbody>
<tr class="roadmap-milestone-field"><th scope="row">Outcome</th><td colspan="1">On a sage-dev branch, a separate Anchor Book replaces the provisional anchor markup, passes the structural check, and renders through sage-dev's commands on its pinned nightly toolchain alongside its other mdBook preprocessors; a fresh agent session starts from it.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Status</th><td colspan="1">Proposed</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Dependencies</th><td colspan="1"><a href="#project-integration-2">Milestone 2</a>, whose project setup it reuses.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Design basis</th><td colspan="1"><a href="../design/project-integration.html">Project Integration</a>, including <a href="../design/project-integration.html#what-the-crate-must-not-impose">what the crate must not impose</a>.</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Identifier</th><td colspan="1"><code>project-integration-3</code></td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Tasks</th><td colspan="1"><ul class="roadmap-tasks">
<li><label><input type="checkbox" disabled> Build and run the pinned engine revision with sage-dev's pinned nightly toolchain; fix or record any incompatibility.</label></li>
<li><label><input type="checkbox" disabled> Create the Anchor Book from sage-dev's design material, as in milestone 2, and replace its provisional anchors with Anchor Book records. Removing the provisional tooling is the maintainers' decision.</label></li>
<li><label><input type="checkbox" disabled> Render with the Anchor Book preprocessor in the Anchor Book's own <code>book.toml</code>; confirm the user Book and its preprocessors are unaffected.</label></li>
<li><label><input type="checkbox" disabled> Exercise a fresh agent session and record gaps as in milestone 2; fix them in this Book first.</label></li>
</ul></td></tr>
<tr class="roadmap-section"><th colspan="2">Completion criteria</th></tr>
<tr><th scope="col">Criterion</th><th scope="col">Complete when</th></tr>
<tr id="project-integration-3-nightly-toolchain" class="roadmap-criterion-summary"><th scope="row">Nightly toolchain</th><td><p class="roadmap-condition">The engine builds and checks the Anchor Book on sage-dev's pinned nightly toolchain without changing sage-dev's toolchain or dependencies.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for nightly toolchain" aria-expanded="true" aria-controls="project-integration-3-nightly-toolchain-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-3-nightly-toolchain-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>nightly-toolchain</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-no-imposed-dependencies">project-no-imposed-dependencies</a> · <code>dependency-review</code> — Partial: one pinned nightly toolchain.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-3-native-commands" class="roadmap-criterion-summary"><th scope="row">Native commands</th><td><p class="roadmap-condition">sage-dev's documented commands check and render the Anchor Book, its user Book and preprocessors are unaffected, and a deliberately broken copy is not rendered.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for native commands" aria-expanded="true" aria-controls="project-integration-3-native-commands-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-3-native-commands-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>native-commands</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-native-commands">project-native-commands</a> · <code>consumer-fixture</code> — Partial: a second project with other preprocessors.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr id="project-integration-3-agent-start" class="roadmap-criterion-summary"><th scope="row">Agent start and gaps</th><td><p class="roadmap-condition">A fresh agent session starts from the contract and the Anchor Book, and every gap found is fixed or recorded, as in milestone 2.</p><button type="button" class="roadmap-toggle" aria-label="Anchor details and evidence for agent start and gaps" aria-expanded="true" aria-controls="project-integration-3-agent-start-details"><span class="roadmap-toggle-icon" aria-hidden="true">▾</span> Anchor details and evidence</button></td></tr>
<tr id="project-integration-3-agent-start-details" class="roadmap-criterion-details"><td colspan="2">
<p><strong>Criterion:</strong> <code>agent-start</code></p>
<p><strong>Anchor criteria and planned coverage:</strong> <a href="../design/project-integration.html#project-agent-bootstrap">project-agent-bootstrap</a> · <code>fresh-session</code> — Partial: second project.</p>
<p><strong>Evidence:</strong> Pending.</p>
</td></tr>
<tr class="roadmap-milestone-field"><th scope="row">Completion record</th><td colspan="1">Pending.</td></tr>
</tbody>
</table>
</div>
