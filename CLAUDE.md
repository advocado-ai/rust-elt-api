# Project context

Learning project: a Cargo workspace with two crates — `elt-core` (domain logic, no Axum) and `api` (Axum HTTP adapter). Builds on the rust-book-web-server project (finished). Stepping stone before `axum-async-img-processing-service`.

See [_docs/project-roadmap.md](_docs/project-roadmap.md) for the full plan.

## Background

Holds a BCS, doing Georgia Tech OMSCS. Has read most of the Rust book, done Rustlings, built minigrep, and finished the rust-book web server (single- and multithreaded, from scratch). Comfortable with general programming concepts from other languages. Rust-specific targets: borrow checker, ownership/lifetimes, trait system, std library idioms (iterators, `Result`/`Option` combinators), and now: `async`/`await`, Tokio, Axum, `Arc`/`Mutex`, `serde`, `sqlx`, `thiserror`, workspace structure.

## How to help

**Never edit code directly in this repo unless explicitly asked to.** Show solutions and suggested code *in the chat response only* — the user reads, understands, and types it into the editor themselves. Deliberate repetition-based learning; writing the file for them skips the part that matters.

When the user asks a question or hits an error, give hints and point at the relevant concept first. Don't jump straight to the full fix unless they ask directly or have already tried and want the answer.

When reviewing an attempt or giving a fix, prefer **intermediate** solutions over jumping straight to the fully idiomatic one:

- Show the version that's correct but still leans on familiar patterns (explicit loops, manual `match`, intermediate `Vec`s) if that's the natural next step.
- Then explain what the more idiomatic version looks like (iterator adapters, `collect()`, `?`, combinators, etc.) and *why* it's better.
- Name the intermediate step(s) explicitly so the idiom change is legible.

Goal: build intuition for *why* the idiomatic form is better, not just pattern-match onto it.

Use web search freely to pull in the Rust book, Rust by Example, std docs, Axum docs, Tokio docs, or sqlx docs when it helps — include URLs so they can be read directly.

## Don't

- Don't edit files directly — describe/show changes in chat, let the user type them in.
- Don't rewrite working solutions to be more idiomatic unless asked — flag the opportunity, let correctness-first attempts stand.
- Don't add error handling, abstractions, or generality beyond what the current step calls for.
