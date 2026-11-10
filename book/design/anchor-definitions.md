# Anchor Definitions

This chapter defines the record syntax, a strict line grammar.



An anchor defines a property of the goal system, its authority, and verification
criteria. Surrounding prose explains it but is not part of the definition. Agents
search, read, and edit Book sources rather than the rendered Book, so the syntax
puts each record's identifying facts on one line and spells every reference one
way. The [record library and renderer](record-rendering.md) present the same
records readably.

## Anchors

```anchorbook-anchor
schema-version = 1
id = "records-searchable-lines"
title = "A record's identifying facts are on one line"
status = "draft"
property = '''
Every record's heading line states its identifier, status, and title; the
identifier's kind code states its kind; and every reference to a record is its
code-spanned identifier. A text search for a record's identifier returns its
definition and every reference to it, and the matched line alone distinguishes
the definition from a reference.
'''
authorities = [
  { kind = "requirement", id = "requirement-searchable-records" },
]

[[criteria]]
id = "heading-search"
verification = ["test"]
statement = "For an independent fixture Book, a search for record headings returns exactly the authored records with their kind, identity, status, and title."

[[criteria]]
id = "reference-search"
verification = ["test"]
statement = "For each fixture record, a search for its identifier returns its definition and exactly its authored references, and each matched line is classified correctly from its own text."
```

```anchorbook-anchor
schema-version = 1
id = "records-every-line"
title = "No line in a record is silently ignored"
status = "draft"
property = '''
Each line of a record is read as its heading, a field, a continuation paragraph,
a list item, a proof-scope item, or a blank quoted line. Any other line, and any
missing, repeated, unknown, or misordered field, is an invalid definition reported
at its source line, so a mistyped record cannot pass as valid.
'''
authorities = [
  { kind = "requirement", id = "requirement-readable-records" },
]

[[criteria]]
id = "line-accounting"
verification = ["test"]
statement = "Fixtures with stray, misordered, repeated, unknown, and missing lines each produce the expected located diagnostic and no inventory; the corrected fixture produces the expected record."
```

```anchorbook-anchor
schema-version = 1
id = "records-named-identifiers"
title = "Record identifiers state their kind and name their meaning"
status = "draft"
property = '''
Every anchor, requirement, and decision identifier is its kind code, a hyphen,
and a name that starts with a letter, so an identifier seen alone in a
reference, evidence link, fragment, diagnostic, or search result states its
record's kind and cannot be a sequence number. Each reference field accepts only
the codes of the kinds it may cite. An identifier or reference violating these
forms is an invalid definition reported at its source line.
'''
authorities = [
  { kind = "requirement", id = "requirement-searchable-records" },
  { kind = "decision", id = "decision-identifier-kind-codes" },
]

[[criteria]]
id = "identifier-forms"
verification = ["test"]
statement = "Fixtures with a missing, unknown, or lowercase kind code, a code that does not match the record's kind, and names starting with a digit, such as D-12 or R-2026-01, each produce a located invalid-definition diagnostic and no inventory; names with digits inside a word, such as D-utf8-paths, are accepted."

[[criteria]]
id = "reference-codes"
verification = ["test"]
statement = "An Authority field citing an anchor identifier and a Requirements field citing a decision identifier each produce a located invalid-definition diagnostic, even when the cited record exists."
```

## Record syntax

A record is one Markdown blockquote: consecutive lines beginning with `>`. Its
first line is the heading; labeled field lines follow. One blockquote holds
exactly one record and nothing else, so the record's source range is the whole
blockquote. There are no inline or heading-based definitions, alternate forms,
inferred fields, or aliases. Literal examples go in code blocks, which are never
records:

```markdown
> **`A-check-observation-only`** (draft): **Checking preserves the Book for deliberate repair**
> Property: Engineers and agents can inspect a Book's format without changing the project or triggering its tools.
> Repairs remain separate actions so the engineer or agent controls what changes.
> Authority: `R-observation-only`; `D-separate-check-and-repair`
> - Criterion `file-preservation` [test]: Compare project file contents and directory entries before and after passing, failing, and incomplete checks; they remain unchanged.
```

### Heading

``**`<id>`** (<status>): **<Title>**``

| Part | Constraint |
| --- | --- |
| Identifier | An [identifier](#identifiers), unique across this Book's records; its kind code states the record's kind |
| Status | Exactly `draft` or `accepted`; this is design acceptance only |
| Title | Nonempty, for display |

A blockquote whose first line starts with `` **` `` but does not match this form
is an invalid definition, not prose.

### Identifiers

An identifier states its record's kind and names what the record means: the
property an anchor guarantees, the outcome a requirement demands, or the choice
a decision makes. It appears without the record in references, evidence links,
link fragments, diagnostics, and search results, so an engineer or agent must be
able to recognize the record from the identifier alone. The identifier is the
only place a record's kind is stated.

An identifier is a kind code, a hyphen, and a name:

| Kind | Code | Name states | Example |
| --- | --- | --- | --- |
| Anchor | `A` | The guaranteed property, led by its capability word; not its test or implementation | `A-check-observation-only` |
| Requirement | `R` | The required outcome or constraint | `R-observation-only` |
| Decision | `D` | The selected approach, not the problem it addresses | `D-separate-check-and-repair` |

The name consists of lowercase ASCII letters and digits in hyphen-separated
words, starts with a letter, and has no trailing hyphen; the whole identifier
matches `^[ARD]-[a-z][a-z0-9]*(-[a-z0-9]+)*$`. Because the name starts with a
letter, an identifier cannot be a sequence number: `D-12` and `R-2026-01` are
invalid, while digits inside a word, as in `D-utf8-paths`, are allowed.

The capability word is shared by the anchors that constrain the same engine or
system capability, such as `check`, `records`, `render`, or `verify`. The
project's design chapters establish its capability words. It is not the chapter
path, so moving a chapter does not rename its anchors.

A criterion identifier has no kind code: it is a name, unique within its anchor,
stating what the verification establishes, such as `file-preservation`. It does
not repeat its anchor's capability word, because evidence links join them as
`<anchor-id>#<criterion-id>`.

Keep names precise and concise: as short as they can be while unambiguous, most
of them two to five words. Do not encode dates, version numbers, statuses,
owners, or chapter locations.

**Writing identifiers.** Write an identifier in a code span wherever it appears,
in records and prose alike, so that a search for `` `D- `` finds every intended
mention and ordinary words such as "R-squared" never match.

**Stability.** Identity does not depend on a record's title, file, or position.
Retitling or moving a record keeps its identifier. A draft identifier that
misdescribes its record can be corrected together with every reference, evidence
link, and link to its fragment. When a record's meaning changes, it becomes a
new record with a new identifier rather than a reworded old one. An identifier
is never reused for a different property, outcome, or choice.

**What the engine checks.** The reader rejects an identifier that does not match
the form above, and a reference whose kind code its field does not accept.
Whether a name accurately describes its record remains a review responsibility.

### Anchor fields

Each field line begins with its label and a colon. Every field is required, in
this order, with no defaults:

| Field | Constraint |
| --- | --- |
| `Property:` | Nonempty; the complete property, including any qualifications |
| `Authority:` | One or more references, separated by `;` |
| Criteria | One or more criterion items |

**Paragraphs.** Each paragraph is one source line. An unlabeled line continues the
field above as a new paragraph. Blank quoted lines (`>`) are allowed and carry no
meaning.

**References.** A reference is a code-spanned identifier, such as
`` `R-observation-only` ``. `Authority:` accepts `R-` and `D-` identifiers, each of
which must resolve to a record of that kind in the same Book. Duplicate
references are errors. Both kinds are allowed but not required. Cross-Book
references are outside this syntax.

**Criteria.** A criterion is a list item ``- Criterion `<id>` [<methods>]: <statement>``.
Its identifier is unique within its anchor, and its statement is a nonempty
description of what verification must establish. [Evidence links](verification-evidence.md#evidence-links)
reference the anchor identifier and criterion identifier together.

**Every line accounted for.** Each line is the heading, a field, a continuation, a
criterion, a proof-scope item, or a blank quoted line. Any other line, and any
missing, repeated, unknown, or misordered field, is an invalid definition at its
source line.

### What one search answers

| Question | Search |
| --- | --- |
| Every record, with identity, status, and title | ``^> \*\*`[ARD]-`` |
| Every decision, with status and title | ``^> \*\*`D-`` |
| Draft anchors | ``^> \*\*`A-.*\(draft\)`` |
| Every mention of any decision | `` `D-[a-z] `` |
| A record's definition and every reference to it | `` `R-observation-only` ``; the definition is the match on a line beginning `` > **` `` |
| Criteria requiring proof | `Criterion .*\[[^]]*proof` |

Literal examples in code blocks also match text searches; they are not records.

### Verification methods

Every criterion names a nonempty, comma-separated list of distinct methods in
brackets. The only values are `proof`, `test`, and `review`, with exactly that
spelling. Every listed method is required; the list is not a choice between
alternatives. Order has no semantic significance and is preserved for
presentation. Missing, empty, repeated, or unknown methods are invalid definitions.

| Value | Required evidence |
| --- | --- |
| `proof` | Formal verification of the relevant production code against an explicit specification expressing the criterion, with assumptions and coverage stated. |
| `test` | Executed tests whose assertions derive from the criterion, with inputs, results, and coverage recorded. |
| `review` | A recorded assessment against the criterion, identifying what was examined, the conclusion, and limits. |

Different criteria within an anchor may require different methods. Listing proof
does not make an anchor accepted or verified. Methods declare obligations; parsing
and rendering them do not run tools or establish evidence. Concrete tool choices
and commands belong in the verification design and roadmap, not in this field.

For example, one criterion can require a proof of the validation algorithm while
another requires tests and review of its integration. If a method covers only a
portion, identify that portion and the remaining obligations explicitly; partial
evidence does not satisfy the whole criterion.

### Does an anchor require formal verification?

Determine this from the criterion declarations, for the current anchor revision:

| Declaration | Meaning |
| --- | --- |
| At least one criterion lists `proof` | Formal verification is required for every criterion that lists it, within that criterion's stated scope. |
| Every criterion has a valid method list and none lists `proof` | Formal verification is not required; the listed tests and reviews remain required. |
| A method list is missing or invalid | The definition is invalid. Do not interpret missing information as “proof not required.” |

Do not add a separate anchor-level proof flag or infer proof requirements from
the anchor's title, acceptance status, or the presence of proof code. The method
lists are the single source of truth. Requiring proof for one criterion does not
mean the entire anchor is formally proved, nor does it describe a completed run.

Choose methods during design review, following
[Choosing methods](verification.md#choosing-methods), and record the rationale
with the criterion's design.

### Required scope of a proof criterion

A criterion listing `proof` states its claim as the statement and carries exactly
four nested items, in order:

```markdown
> - Criterion `formal-authority` [proof]: The production authority validator reports no violations if and only if every decoded record satisfies the authority rules.
>   - Domain: The complete typed candidate collection, including empty lists, duplicates, and missing, invalid, or incompatible targets.
>   - Boundary: Executable authority-presence checks, target lookup, and validation across all records.
>   - Assumptions: The verifier, solver, compiler, and standard-library contracts are trusted.
>   - Exclusions: Parsing and candidate conversion, covered by `production-integration`.
```

| Item | Required content |
| --- | --- |
| Statement (Claim) | The property the proof must establish, including acceptance and rejection when both are required. |
| `Domain:` | The inputs, states, conditions, and bounds covered. Include relevant invalid cases; do not assume the property being proved. |
| `Boundary:` | The production behavior covered, such as the validation algorithm or the path from typed input to the public API result. |
| `Assumptions:` | Caller obligations and trusted components or contracts. Preconditions must not assume the claim itself. |
| `Exclusions:` | Behavior outside the proof, with the criteria that cover it through tests, review, or another proof. An uncovered obligation remains an explicit gap. |

Exclusions name each covering criterion by its code-spanned identifier: the
criterion identifier alone for one of the same anchor, or
`<anchor-id>#<criterion-id>` for another anchor's. The engine checks that the four
items are present and nonempty and that each named criterion exists; it does not
assess whether the scope is adequate or whether evidence satisfies it. The
[brief](verification-evidence.md#using-verify) lists exclusions that name no
criterion as gaps. Keep formal syntax,
tool versions, commands, and function/file associations in the implementing
codebase and verification design. Exclusions must not silently remove part of the
anchor's property. The [authority anchor](rust-engine.md#records-valid-authority)
demonstrates this: `formal-authority` requires a proof of the authority algorithm,
while `production-integration` requires tests and review of the surrounding boundaries.

### Displaying verification requirements

The renderer displays **Required verification** beside each criterion. It must
not turn these selections into success badges or inferred results.

### Writing properties for verification

State the behavior, relevant conditions, and required outcome precisely in the
property; define terms in the surrounding design. Criteria state what evidence
must establish. Keep implementation steps, tool commands, and proof mechanics in
their owning design and roadmap. The working contract defines the agent's
verification responsibilities (`anchorbook contract`, “Develop verification with the code”).

Formal specifications, production code, and supporting proofs live together in
the implementing codebase, associated with criteria through
[evidence links](verification-evidence.md#evidence-links); do not add anchor fields
for file paths, proof text, results, or timestamps.
[Proofs that match the design](verification.md#proofs-that-match-the-design)
defines how a proof criterion's scope becomes a specification. Assumptions that
narrow the intended property require design review; assumptions about unverified
implementation boundaries must remain visible in evidence.

### Value validation and syntax evolution

Whitespace-only text is empty. Do not infer missing values from surrounding prose
or accept misspelled kinds, statuses, labels, or methods. Changes to these rules
require an explicit revision of the syntax rather than reader leniency. The syntax
is an unpublished draft that can be revised in place with this Book's records;
before publication, a compatibility and migration policy must define how consuming
Books identify the syntax version they use. The [authority syntax](authority-definitions.md#evolving-the-authority-model)
remains provisional and will evolve through use.

## Parsing and validation

The shared engine library reads records with a line scanner over chapter text,
retaining source lines and byte ranges. Markdown parsing is used only to identify
code regions, so text inside fenced or indented code blocks is never a record.
The library does not extract definitions with regular expressions over whole
chapters or infer structure from headings.

Reject a malformed heading, an unknown kind, status, label, or method, a missing,
repeated, or misordered field, a stray line, duplicate identity, or unresolved
authority with a diagnostic at the source location. Never silently discard an
invalid recognized definition.

Resolve references after collecting the Book inventory, so chapter order does
not affect validity. An accepted anchor must cite accepted authority; draft
anchors may cite draft authority while the design is being reviewed. Reference
validity does not establish that an authority logically justifies the property;
that relationship still needs review.

[Typed authority definitions](authority-definitions.md) supply reference identity;
headings and HTML targets do not. Accepted decisions must also cite only accepted
requirements. The shared index checks all these relationships.

The library owns these rules. The mdBook preprocessor consumes the same validated
model to render anchors and links. Rendering must not define a second
interpretation of the source.

## Design, implementation, and evidence

`accepted` status means the property has been accepted into the goal design.
It does not mean implemented, tested, or proven. Keep implementation associations,
executed results, and roadmap completion outside the authored definition.
Reports derive progress from those records.

## Open questions

1. **Inline Markdown in fields.** Whether field text renders a defined subset
   (code spans, emphasis, links) with raw HTML kept inert, or stays escaped text.
2. **Property label.** Whether `Property:` is required or the first unlabeled line
   after the heading is the property.
3. **Proof-scope shape.** Whether the statement as Claim and four nested items fit
   real proof criteria.
4. **Example matches.** Whether literal examples matching record searches mislead
   agents, and if so whether examples need a visibly distinct form.
