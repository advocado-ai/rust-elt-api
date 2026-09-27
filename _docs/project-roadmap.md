# ELT + Axum Backend Roadmap

## Goal

Build a Cargo **workspace** with two crates:

```text
rust-elt-api/
  Cargo.toml          workspace root
  elt-core/            domain: record types, transform logic, repository trait
  api/                 Axum HTTP adapter around elt-core
```

`elt-core` knows nothing about Axum, Tokio, or HTTP. It is a plain library
crate: types, transforms, and a repository trait, fully covered by ordinary
`#[test]` unit tests with no server involved. `api` depends on `elt-core` and
is a thin adapter: deserialize a request, call into `elt-core`, serialize a
response.

```text
HTTP handlers (api) -> elt-core services/domain types
                                          -> elt-core repository trait
                                                       -> in-memory impl, later sqlx impl
```

Writing `elt-core` first (and testing it in complete isolation, no HTTP, no
async required for the core transform logic) gives a fast synchronous test
loop for the part that matters most and reuses the Python/Prefect ELT
background directly: shape a Mongo-like source document, normalize it into a
target record, validate/track status. `api` is deliberately boring by
comparison — that's the point, since Axum itself is the new material there.

This backend is separate from `rust-cli-elt`. It may reuse domain ideas from
that project's future ELT logic, not copy the CLI wholesale.

## Part 1 — `elt-core` (domain crate, no Axum)

Goal: prove the ELT logic works with plain `cargo test`, before any web
framework enters the picture.

1. `cargo new --lib elt-core` inside a new workspace; add workspace
   `Cargo.toml` with `members = ["elt-core", "api"]`.
2. Define domain types: a source record shape (Mongo-like document), a
   normalized target record, an `ImportStatus` enum.
3. Write the transform function(s): source record -> normalized record.
   Cover edge cases (missing fields, type mismatches) with unit tests.
4. Define a repository trait (e.g. `ImportRepository`) with methods like
   `save_import`, `get_import`, `list_records` — no implementation detail
   leaks through the trait.
5. Implement the trait in-memory (`HashMap` behind the struct, no `Arc`/
   `Mutex` needed yet since there's no concurrency at this layer).
6. Add structured domain errors with `thiserror` (e.g. `TransformError`,
   `NotFound`).
7. Unit-test the whole crate: transform correctness, repository behavior,
   error paths. This crate should reach high test coverage before Part 2
   starts.

### Rust practice targets (Part 1)

- Ownership and borrowing across functions
- `Result`/`Option`, `?`, custom error types with `thiserror`
- Traits and trait objects (or generics) for the repository boundary
- Unit testing idioms (`#[cfg(test)]`, table-driven cases)

## Part 2 — `api` (Axum adapter crate)

Goal: expose `elt-core` over HTTP. Handlers stay thin; logic stays in
`elt-core`.

### First API surface

```text
GET  /health        health check
POST /imports       submit an import
GET  /imports/:id   inspect import status
GET  /records       query normalized records
```

1. `cargo new api` (binary crate) inside the workspace; add `elt-core` as a
   path dependency. Add a `GET /health` endpoint first.
2. Add request/response DTOs with `serde` in the `api` crate (keep them
   separate from `elt-core`'s domain types — convert at the boundary).
3. Wire `POST /imports` and `GET /imports/:id` to the in-memory
   `ImportRepository` from Part 1, shared across handlers with `Arc`.
4. Add `GET /records`.
5. Map `elt-core` errors to HTTP responses (`IntoResponse` impls per error
   variant).
6. Swap the in-memory repository for a `sqlx`-backed SQLite or Postgres
   implementation of the same trait defined in `elt-core` — handlers and
   routes don't change, only the implementation passed in at startup.
7. Add configuration, `tracing`, graceful shutdown, and integration tests
   (`reqwest` or Axum's test utilities against the real router).
8. Add background jobs and deployment concerns after the core path is
   reliable.

### Rust practice targets (Part 2)

- Axum extractors and response types
- Tokio tasks and async functions
- Shared state with `Arc` (and `Mutex`/`RwLock` if the in-memory repo needs
  interior mutability)
- Database access with `sqlx`
- Serialization with `serde`
- `Send` and `Sync` requirements for async shared state
- Error conversion at application boundaries

## Suggested dependencies

`elt-core`:
- `serde` for domain type (de)serialization support
- `thiserror` for domain/application errors

`api` (additionally):
- `axum` for HTTP routing
- `tokio` for the async runtime
- `serde_json` for JSON
- `sqlx` for database access
- `tracing` and `tracing-subscriber` for observability

## Showcase checklist

- README includes a runnable quick start
- Sample `curl` requests are documented
- `elt-core` unit tests cover domain transformations and repository behavior
  in isolation (no server required)
- `api` integration tests cover the HTTP contract end to end
- `cargo fmt`, `cargo clippy`, and `cargo test` pass across the workspace
- Architecture notes explain the crate boundary, ownership, and concurrency
  choices
- Configuration and shutdown behavior are documented
