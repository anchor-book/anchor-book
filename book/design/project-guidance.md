# Project Guidance and Terminology

A Book gives people and agents both a shared working method and the context to
apply it to a particular project. The AI Working Contract supplies the method;
project guidance supplies local instructions; terminology makes their vocabulary
explicit. Consumers maintain their own instructions and domain terms while
Anchor Book maintains its standard terms.

The ownership guarantee is
[A-materialize-owned-content](system-overview.md#A-materialize-owned-content).
The [agent bootstrap](project-integration.md#A-project-agent-bootstrap) connects
the contract to these pages and the rest of the Book. Section names and individual
terms need no separate anchors.

## Project guidance

Project guidance is an engine-managed Book role for developer workflow, review
guidance, and project logistics. Anchor Book owns the role, starter template, and
section responsibilities; the project owns the page's instructions. The project
chooses its title and path; “Working on …” is one suitable title. Configure it as
`book.sections.project-guidance`, link it at the top level of `SUMMARY.md`, and
identify it in the repository bootstrap. It is distinct from design, authority,
and implementation roadmaps. Projects can adapt headings, add sections, and link
detailed instructions maintained elsewhere.

| Section | What the project supplies |
| --- | --- |
| Project context | What is being developed, links to its purpose and scope, and the authoritative phase setting. |
| Book sources and references | Configuration and source locations, generated output, the terminology page, and authoring references for the selected engine version. |
| Developer workflow | Supported setup, development, structural checking, rendering, and verification commands, with tool versions and limitations. Link contributor instructions when they already own these details. |
| Design and review workflow | Where intended design, research, open questions, proposals, approvals, and audit reports belong; how to review design, anchors, code, and evidence; any conventions for diagrams and examples. |
| Project logistics | Branch and contribution workflow, delivery and preview instructions, ownership, and confidentiality and publication boundaries. |

Keep each fact in one authoritative place. Guidance points to configuration for
the current phase, roadmaps for work and evidence, and contributor instructions
for exact commands. It does not repeat the reusable contract, maintain a second
readiness template, or turn planned commands into available tools. Missing
information is an explicit gap; a section may explain that a capability is not
configured rather than inventing instructions.

Agents maintain this page as local practices change, using the engineer's
existing choices and authorization. They raise consequential policy changes
with the engineer. Project guidance remains subject to the agent's instruction
hierarchy; it does not override an instruction from the engineer or make quoted
source material into instructions.

Review procedures belong here; individual audit findings and executed results
remain in linked reports or milestone evidence. Design chapters define intended
behavior, and roadmaps define work and completion evidence. A review guide cannot
substitute for an implementation roadmap. A project with no planned milestones
can say so in its separate roadmap index.

## Setup and compatibility

`anchorbook template project-guidance` prints the engine-versioned Markdown
starter to standard output without reading configuration or changing files.
An agent adapts it using known project practices, preserving existing instructions
and identifying missing information. It is not an initialized project policy.
For an existing page, reconcile its content with the section responsibilities;
never redirect the template over authored content. The command rejects unsupported
template names and arguments without printing a partial template.

New Books and assigned adoption or reconciliation work establish the guidance
page and its mapping. Schema 1 accepts the optional `project-guidance` field so
existing Books remain readable during migration. Omission does not create a page,
infer one from its title, or establish that project guidance is available. Agents
report the gap and establish guidance within the assignment. Once mapped, the
page must resolve within the source root, be distinct from every other mapped
role including symlink aliases, and appear as a top-level local chapter. The
ordinary chapter checks require its file to exist. Content quality, section
completeness, and the appropriateness of project policies remain agent and human
review obligations; a structural pass does not assess them.

Upgrade every engine entry point before adding the field; earlier engines reject
it as unknown. Keep the existing schema number because the additional field is
optional and existing configurations retain their meaning. This does not enable
the separate draft RFD schema. Contract retrieval remains reusable text plus
phase guidance; it neither includes nor executes the project's instructions.
The agent finds the page through the mapping, relative to the configured Book
source, and reads it separately. Checking, template retrieval, and contract
retrieval never create, replace, or synchronize project guidance.

Automatic materialization and standard terminology delivery are separate work.
The terminology page's mandatory status and mapping remain
[adoption choices](../open-questions.md#guidance-and-terminology).

## Terminology

A top-level terminology page has two separately owned tables. Each entry gives
a term, a concise meaning, and a link to its defining material when available.
The definition remains authoritative; the table helps readers find and use it.
Keep an existing glossary filename when it serves this purpose, preserving its
links. Project guidance identifies the page and its place in top-level navigation.

| Table | Owner | Contents |
| --- | --- | --- |
| Anchor Book terminology | Anchor Book, versioned with the engine | Shared concepts such as design, authority, anchor, criterion, phase, and milestone, with meanings consistent with that version's contract and authoring references. |
| Project terminology | The consuming project, maintained by its agents and engineer | Domain concepts, component names, and technical terms needed to understand that project's system. |

Project terms extend the shared vocabulary. A project may use a qualified name
for a domain meaning that would otherwise collide with a standard term, such as
“function contract” alongside “AI Working Contract.” Do not silently redefine a
standard term in the project table. Surface a real terminology conflict for the
engineer's judgment and reconcile affected prose and identifiers once agreed.
Existing approvals remain valid; ordinary wording corrections need no new
approval ritual.

Standard terms must remain portable: their definitions cannot depend on a
consumer's directory layout, selected language, or private project context.
References resolve to the matching engine version's authoring material, using
the reference-delivery mechanism shared with contributor setup. A consumer's
own design chapters define its project terms. For Anchor Book's own Book, that
design also defines the standard concepts, so local links can serve both purposes.

Optional or proposed concepts stay visibly conditional. Listing RFD terminology,
for example, neither enables RFD support nor makes that proposal an accepted
capability. General technical vocabulary belongs in the project table when it
is specific to understanding that system; not every word needs an entry.

### Authoring and maintenance

Use the selected engine version's standard definitions and record that version
when copying them into a consumer Book. Resolve their links to that version's
authoring material rather than copying this repository's relative paths into
another project. If references are unavailable, identify the gap; do not invent
definitions or claim the copy is current. Project entries link to the consumer's
own definitions. A changed definition is reviewed in its owning design before
the table is reconciled; editing the glossary alone does not change a requirement,
anchor, or accepted decision.

On upgrade, compare standard terms with the newly selected engine version and
reconcile the standard table and its references. Preserve project terms and
surrounding prose; surface collisions or local edits to shared meanings for
resolution. A project-specific sense gets a qualified name and its own entry.
For example, “record status” describes design acceptance, “milestone status”
describes work progress, and an evidence state describes verification. None
substitutes for the others. Keep obsolete terminology only when a compatibility
note helps readers find the current definition.

This structure supports manual adoption without a new configuration key.
The guidance template asks for the terminology location; it does not supply
standard definitions or create the page. Engine installation, contract retrieval,
and structural checking do not update tables or assess their meanings. Packaged
terminology delivery and automatic updates have their own implementation work.

## Ownership through updates

Updating Anchor Book updates the standard table for the project's selected
engine version. It preserves the project table, local guidance, and surrounding
project-authored content. Updating a project term does not change the standard
table or the contract. Local guidance and domain terms are part of the project's
authored content even when an agent wrote them or a template supplied the initial
headings.

Ownership must be explicit before automation writes a page. A heading or table
position alone is not permission to replace content. Missing or ambiguous
boundaries, conflicting local edits to standard content, and version mismatches
must be exposed for reconciliation rather than resolved by overwriting local
work. An already-conforming update produces no changes. These are applications
of the existing materialization guarantee, not a separate synchronization policy.

The agent reads the contract, project guidance, and Book, including both tables.
Contract retrieval supplies reusable rules; it does not fetch, merge, or update
project pages. Agents follow the declared ownership during manual maintenance
as well as when using a supported materialization operation.
