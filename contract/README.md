# AI Working Contract sources

These files are the maintained source of the AI Working Contract, the reusable
guidance agents read before working on any Anchor Book project. The engine joins
them, in the order listed in `src/contract.rs`, into the text `anchorbook contract`
prints. Edit the section files here; there is no other copy. Keep project-specific
content out of them: it belongs in each project's Book.

Each file holds whole top-level sections. The first begins with the contract's
title. Links between sections use their heading fragments, which stay valid
because the joined text is one document.
