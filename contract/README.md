# AI Working Contract sources

These files are the maintained source of the AI Working Contract, the reusable
guidance agents read before working on any Anchor Book project. The engine joins
the common sections, in the order listed in `src/contract.rs`, into the text
`anchorbook contract` prints. `anchorbook contract --config PATH` reads the
configured phase and appends `port.md` only for `port`. Edit the section files
here; there is no other copy. Keep project-specific
content out of them: it belongs in each project's Book.

Each file holds whole top-level sections. The first begins with the contract's
title. Links between sections use their heading fragments, which stay valid
because the joined text is one document.

`guidance.md` equips agents to explain the workflow and offer contextual next steps
and prompt examples. The Book's `design/user-guidance.md` owns the user-flow design
and fuller reader guide; update them together. Consuming projects retrieve the
packaged contract and supply their own context rather than copying either file.
