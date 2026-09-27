# rust-elt-api Documentation

This directory contains project planning and learning notes for the standalone
ELT + Axum backend, built as a two-crate workspace: `elt-core` (domain logic,
testable in isolation) and `api` (the Axum HTTP adapter around it).

- [Project roadmap](project-roadmap.md)

The backend is separate from `rust-cli-elt`. It should reuse the domain ideas,
not copy the CLI wholesale. Axum is an HTTP adapter around testable
application and persistence code living in `elt-core`.
