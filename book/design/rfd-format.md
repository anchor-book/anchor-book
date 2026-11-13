# RFD Source Format

This chapter defines RFD format 1, its engine-owned templates, and its version
boundary. [Requests for Discussion](rfds.md) defines the process and guarantees.
Projects supply their proposals and review policy; the engine owns this syntax.

## Configuration and compatibility

Configuration schema 2 adds the optional `book.sections.rfds` mapping. All
schema-1 fields retain their meanings and the three existing roles remain
required. The engine accepts schema 1 unchanged, without RFD support, and schema
2 with or without that optional mapping. A schema-1 file containing the mapping
is invalid; unknown fields and unsupported versions remain errors.

To opt in, upgrade every project entry point to a supporting engine revision,
change `schema-version` to 2, add the mapping, and add the index and navigation in
one reviewed change. An older engine rejects schema 2 rather than silently
ignoring the capability. Removing the mapping disables RFD processing; ordinary
Book checks continue. Returning to schema 1 also requires removing any
schema-2-only fields. Neither operation rewrites the proposal files.

Each RFD declares `format-version = 1`. A format change that changes the meaning
or validity of existing input requires a new format version and explicit migration
instructions; readers never reinterpret an older version or rewrite it during
checking. Templates identify their format and ship with the engine revision that
reads it. The existing current-record grammar is unchanged; format 1 embeds its
field meanings as defined in [Anchor Definitions](anchor-definitions.md).

## Index and ownership

The configured index is an ordinary Markdown landing page with a brief process
introduction and links to proposals. Its prose and headings have no schema fields.
Grouping links by disposition is optional presentation; disposition comes from
each proposal's metadata. `SUMMARY.md` remains the only chapter hierarchy.
Unfilled template prompts in the index's prose or links are authoring findings;
examples inside ordinary code fences are excluded from that prompt check.

Every direct local child of the mapped index in `SUMMARY.md` is a proposal root.
Its descendants are supporting chapters owned by that proposal. Each root has
exactly one RFD metadata block; the index and supporting chapters have none.
Targetless or external children cannot stand in for proposal roots. A proposal
cannot own another proposal or share a supporting chapter with another root.
Use ordinary links for shared background material.

The collection directory is the configured index's parent. All proposal roots
and their supporting chapters must lie within that directory and appear exactly
once beneath the mapped index in navigation. Alternative directories are allowed
by mapping a different index. Enabled checks inspect Markdown files in this
directory for RFD metadata blocks and diagnose roots omitted from navigation.
They follow the existing containment rules, detect filesystem aliases and cycles,
and report unreadable inputs as incomplete. This scan is never triggered by a
directory name when the mapping is absent.

Only top-level fenced code blocks with the exact info strings `rfd-metadata`
and `rfd-change` carry RFD data in the enabled collection. Blocks nested
inside quotes, lists, or outer example fences are examples. Recognized blocks
with malformed TOML, missing closure, unknown fields, or wrong placement are
errors; they cannot disappear as ordinary code. Other fences remain examples.
Ordinary unfenced record definitions in the collection are errors: authors must
use proposal blocks or put canonical live definitions in their owning chapters.
With support disabled, these two proposal fences remain ordinary code, as in schema 1.
These markers deliberately avoid the legacy `anchorbook-*` fence prefix, which
the current-record reader continues to reject. RFD format support does not
restore the superseded TOML record syntax or change the ordinary record grammar.

## Proposal metadata

Place one `rfd-metadata` TOML block after the root's level-one display title and
before its first level-two section. Its title field is authoritative; the display
title must match. TOML key order is immaterial. Reject duplicate or unknown keys,
invalid types, empty required strings, and duplicate collection entries.
Unfilled `{{...}}` template prompts in structured fields or required sections are
invalid authored input; they cannot satisfy a field or section's presence check.

| Field | Required value |
| --- | --- |
| `format-version` | Integer `1` |
| `id` | `RFD-` followed by the same meaningful lowercase name grammar used by records; unique across the Book's RFDs |
| `title` | Nonempty title |
| `status` | `draft`, `accepted`, `rejected`, `withdrawn`, or `superseded` |
| `base-revision` | Immutable revision of the Book against which the proposal was prepared |
| `anchor-changes` | `proposed` when at least one anchor change block exists; otherwise `none` |
| `affects` | Array of typed current or historical references to affected Book entities; an empty array is explicit |
| `planned` | Array of typed target strings declaring new design sections, roadmaps, or milestones; additions of records are declared by change blocks |
| `milestones` | Array of current, proposed, or historical milestone references; proposed entries resolve against this RFD's `planned` list |
| `supersedes` | Array of existing RFD IDs replaced by this proposal, or empty |
| `sources` | Array of `{ url, revision, note }` historical citations, or empty; never fetched by checking |
| `incorporation` | Array of incorporation entries described below, or empty |

`no-anchor-change-reason` is required and nonempty exactly when `anchor-changes`
is `none`. Proposed requirement or decision changes may still exist in that case.
`review = { revision, basis }` is required for every non-draft disposition and
optional for a draft with prior review context. It identifies the reviewed
proposal revision and a nonempty account or link to the decision. A review field
records a claim; the checker cannot establish human agreement.

A revision is `git:` followed by a full 40- or 64-digit lowercase hexadecimal
object ID, or `sha256:` followed by a 64-digit lowercase hexadecimal snapshot
digest. It identifies already existing content, never the commit containing its
own reference. Neither branch names nor `HEAD` identify immutable content.
Checking validates the representation, without fetching or resolving history.
Import source revisions and review revisions can differ from the Book baseline.

The root has these level-two sections once, in this order: Summary, Motivation,
Proposal, Proposed anchors, Alternatives, Open questions, Implementation.
Each has nonempty content, including an explicit explanation when nothing applies.
Additional detail uses lower-level headings or supporting chapters. Heading and
content checks establish presence, not whether the proposal is adequate.

## Typed references

A reference is `{ target, scope }`, with `revision` required only for historical
scope. `scope` is exactly `current`, `proposed`, or `historical`. The permitted
scopes depend on the field above. Reject extra fields and conflicting duplicates.

| Target string | Resolution |
| --- | --- |
| `design:<path>[#<fragment>]` | A local chapter in the design role, optionally a heading's explicit stable fragment |
| `anchor:<A-id>` | Current or proposed anchor identity |
| `criterion:<A-id>#<criterion-id>` | Criterion within the named anchor |
| `requirement:<R-id>` | Requirement identity |
| `decision:<D-id>` | Decision identity |
| `roadmap:<roadmap-id>` | Existing roadmap identity |
| `milestone:<milestone-id>` | Existing milestone identity |
| `rfd:<RFD-id>` | Proposal identity |

Paths use `/`, are relative to the Book source, and obey its containment rules.
Design chapter identity is its configured path and, if supplied, explicit heading
fragment; a move requires updating those references. Records, RFDs, and roadmap
work keep their existing stable IDs through moves. No identity is inferred from
display titles or generated heading slugs. Planned design paths need not exist;
planned work IDs obey the same grammar as their eventual current counterparts.

Current references resolve exactly once in the current typed inventory. Proposed
references resolve only within the owning RFD's change blocks or planned targets;
they confer no current authority or progress. Historical references validate the
target's syntax and revision, but remain visibly unevaluated against current
content. A missing current reference is an error, never implicitly historical.
Metadata arrays are sets for validation and retain source order for display.

`supersedes` expresses the direction successor to predecessor. Reject self-links
and cycles. A proposal with disposition `superseded` needs an incoming link from
an accepted or subsequently superseded successor; an accepted or superseded
successor's predecessors must be marked superseded in the same change. Chains
must terminate at an accepted successor. A draft can propose supersession without changing
its predecessor's disposition. Review and implementation states remain separate.

## Proposed definitions

Each `rfd-change` fence contains one TOML change. Anchor changes occur in
the Proposed anchors section or beneath it, or in a supporting chapter linked
from that section. Requirement and decision changes use the same format and may
appear alongside them. They do not extend the current-record authority kinds.

| Field | Rule |
| --- | --- |
| `operation` | Exactly `add`, `revise`, or `retire` |
| `id` | Intended record ID, with its existing `A`, `R`, or `D` kind code |
| `reason` | Nonempty explanation of this change |
| `definition` | Required for add/revise: one complete canonical record as a TOML string; forbidden for retire |
| `bindings` | Required for add/revise: typed references binding every outgoing authority reference and cross-anchor proof exclusion; forbidden for retire |

The embedded record's ID must match `id`. Its fields, criteria, required methods,
and proof scope obey the existing record grammar. Criteria of that same embedded
anchor resolve locally; every other record or criterion reference has exactly one
binding of the correct kind, with no missing or unused entries. Proposed authority
bindings resolve to add/revise definitions in this RFD, never a retirement or
another proposal. Current bindings resolve to the current inventory. Enforce the
same acceptance constraints on resolved authority edges, without merging these
definitions into `RecordInventory` or claiming the existing proof covers this
additional resolver. Retain source mappings through TOML string decoding so an
invalid embedded record has a location in the original file.

Only one operation per record ID is allowed within an RFD. Different RFDs may
propose different versions of that ID. A revise or retire targets the record of
that kind at `base-revision`; an add asserts it is new relative to that baseline.
Checking a current snapshot does not establish those historical facts. It can
show that a target currently exists or is absent, but adoption review must compare
the actual baseline, resolve intervening changes, and assess identity reuse.
An already incorporated addition is therefore not a duplicate current record.

Retained proposals must remain readable after current records move or retire.
Their explicit current references must be updated or made historical in a
reviewed reconciliation. Historical bindings preserve the original reference and
revision, but cannot be reported as resolved current/proposed authority. Views
label the resulting validation limit; another adoption must establish fresh
applicability. Bookkeeping edits preserve the reviewed definition and review
reference; changing the proposed guarantees requires renewed review.

## Incorporation and provenance

An incorporation entry has exactly `proposal-revision`, `book-revision`,
`changes`, `destinations`, and `note`. Revisions identify the reviewed proposal
and an existing Book revision containing the definition updates. `changes` is an
array of record IDs from this RFD's change blocks, and `destinations` is an array
of current or historical typed references. A prose-only adoption uses an empty
change list and names its design destinations. The note explains the adopted
scope, including partial adoption or retirements. At least one change or
destination is required; duplicates within an entry are errors.

Record incorporation after the target revision exists; pending local edits are
reported as pending. The engine derives destination-to-RFD provenance from these
entries, avoiding new fields on authority records. Multiple entries may identify
partial adoptions or later corrections. They are traceability claims, not test
results or proof that the entire RFD was adopted. Report uncovered change IDs and
declared destinations, and leave semantic completeness to adoption review.

## Template delivery

The engine repository owns `templates/rfds/index.md` and
`templates/rfds/proposal.md` and packages their text with the binary. The read-only
commands `anchorbook template rfd-index` and `anchorbook template rfd` print those
templates to standard output and return zero on success. Unknown template names
or extra arguments are usage errors with exit 2. Retrieval needs no project
configuration, filesystem writes, network access, or Book build; it does not
enable RFDs. Project agents can save and edit the output within their assignment.

The templates contain conspicuous `{{...}}` placeholders. Replace them, remove
inapplicable sample operations, and add navigation before checking. Placeholders
are authoring prompts, never accepted values or implied facts. Tests substitute
independently specified values, then exercise the resulting Book through the
actual reader/checker. Template retrieval itself neither generates IDs nor claims
a valid proposal. Do not add a separate scaffolding or adoption command here.

The contract teaches the conditional workflow and points to project guidance for
the supporting engine and commands. Its packaged text remains project-independent.
The index introduces the local review policy and links to proposals; the proposal
template supplies the required sections and change form. Rendered views derive
disposition, backlinks, and linked work from the shared model.

## Evaluation boundary

Enabled `check` adds the scope `rfds/v1` alongside `structural/v1`. It evaluates
collection ownership, canonical source format, typed current/proposed references,
and supersession constraints through the library. Invalid source or relationships
produce located findings; inaccessible files or unsupported versions produce
incomplete evaluation. Use the existing exit conventions: 0 for pass, 1 for
completed findings, 2 for invalid configuration or incomplete evaluation.

Report historical targets and semantic review as outside evaluated scope. A
passing RFD check never claims acceptance, completed adoption, implementation,
or verification. The CLI, preprocessor, and verification entry points use the same
Book loading and proposal-isolation rules. A malformed enabled collection cannot
be silently omitted from a successful Book view or verification run. Disabled
support reports no RFD evaluation and continues existing current-record behavior.

The first implementation exposes the typed model, located check report, and
template commands. Agent and rendered progress views consume that model in the
next milestone; this format introduces no report archive or second authored task
list. [Worked examples](rfd-examples.md) exercise authoring and retained proposals.
