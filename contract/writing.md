## Keep the Book precise and concise

Write Book chapters, records, identifiers, and this contract precisely and
concisely. Clarity and precision take priority over brevity. Include enough context to make
the intended meaning unambiguous: what a rule applies to, why it matters, and
any conditions or limits. Remove repetition and words that add no understanding;
do not shorten away requirements, rationale, distinctions, criteria, or evidence.
Word count is not a target.

Keep each rule in its owning section and link to the details where needed. A
short explanation beside a link is useful when it makes the current section
understandable. The contract may rely on system files guaranteed to be present
and accessible to the agent; read those files when directed. During editing,
compare the revised text with the original to ensure important meaning is retained.

## Maintain the Book's relationships

Record a decision when it governs the system being built: its definition, scope,
design, interfaces, verification, or supported environments. State the choice
and what else in the Book must account for it. Keep a choice affecting only one
local detail in its design text, work sequencing in roadmaps, and how the
project works in project guidance. Do not repeat requirements
as decisions or invent decisions to give anchors authority; a requirement can
ground an anchor directly.

State each anchor as a property that serves the functionality's purpose: what
engineers, agents, or other callers must be able to rely on, with precise scope
and limits. Put concrete verification cases in criteria and implementation
mechanics in the supporting design. Preserve essential constraints when reframing
a property; a general statement of usefulness is not a verifiable guarantee.

Give every anchor, requirement, decision, and criterion a precise, concise
identifier that names its meaning, following the identifier rules in the
project's authoring references. Identifiers appear without their records in
references, evidence links, and search results, so the name alone must identify
the record. Never number records. Retitling or moving a record keeps its
identifier, and an identifier is never reused.

Place anchors at the top of a design chapter, after its brief introduction and
before the detailed design. Keep general anchor-authoring guidance here, not
repeated on design pages.
Use the canonical anchor and authority formats for the project's engine version,
identified in its authoring references. Do not introduce alternate markers,
field aliases, or prose extraction conventions. Each anchor needs at least one
authority. Each reference must resolve to a record of the required kind;
headings and ordinary links do not supply authority identity.

Schema and reference validation cannot establish whether an authority justifies
a property. Review that reasoning. When a requirement is
missing, explain why the property must hold and author the requirement alongside
it; do not add one merely to pass validation.

Use the current schema consistently. Propose changes to fields or relationships
through the schema's compatibility and migration policy; reconcile the schema,
affected records, and validation expectations together. An unpublished draft may
evolve under its documented policy. Accepting a record does not freeze its schema.

Change the goal design directly through the project's review workflow, updating
affected design, requirements, decisions, anchors, and roadmaps together. Keep
unresolved alternatives in the experiment that resolves them or outside the Book;
Git history keeps superseded designs. Agreeing on a design change and completing
its implementation are separate steps.

Design chapters define the goal system, not progress toward it. Keep out status,
gaps, milestones, evidence, dates, and history; roadmaps record those.

Distinguish implementation associations, verification mechanisms, executed results,
and roadmap completion. Report which checks ran, their scope, and results.
Missing, skipped, stale, or inconclusive evidence is not a pass.

