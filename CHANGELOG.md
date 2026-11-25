# Changelog

## [Unreleased]

## [0.1.0]

### Added

- Library and CLI for schema-1 Book configuration, structural checks, design records, authority relationships, roadmap rendering, and verification reports.
- Packaged AI Working Contract, conversational workflow guidance, phase guidance, and a project-guidance template. Markdown prose follows one source line per paragraph.
- mdBook 0.5.4 integration and Rust 1.88 minimum.
- Repository-only proof dependencies, excluded from the distributed manifest, lockfile, and public feature set. Default and all-feature builds need no Verus.

This is a prototype release. Optional RFD support, automatic project creation, version mismatch enforcement, a shared public CLI entry point for workspace binaries, and complete proof-criterion guards are not implemented. Native proof-target success alone does not satisfy proof criteria. See the Book's roadmaps for coverage and remaining work.

The first registry release has no previous published API baseline. Future releases use release-plz's standard library API checks; CLI, configuration, record-format, and contract compatibility also need explicit release notes and migration review.
