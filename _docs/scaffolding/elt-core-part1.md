# `elt-core` Part 1 Scaffold

Read top-to-bottom. Each section maps to one file you create.
Type the code yourself — that's the point. This doc is the answer key.

---

## File layout to create

```
elt-core/
  Cargo.toml          ← add serde + thiserror here
  src/
    lib.rs            ← pub mod declarations only
    types.rs          ← all domain types
    error.rs          ← TransformError + RepositoryError
    transform.rs      ← RawMatch → MatchRecord
    repository.rs     ← trait + InMemory impl
```

---

## Step 1 — `elt-core/Cargo.toml`

Add dependencies. The crate already has `[package]`; just add the block:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
thiserror = "2"
```

No `chrono` yet — store the date as a plain `String` for now and revisit
when you parse it. Keeping dependencies minimal is a good habit.

---

## Step 2 — `src/lib.rs`

Replace the generated `add` function with module declarations:

```rust
pub mod error;
pub mod repository;
pub mod transform;
pub mod types;
```

`pub` on each mod means consumers of the crate can reach into them.
You'll add re-exports here later once the API crate needs them.

---

## Step 3 — `src/types.rs`

These are the core domain types. No logic here — just shapes and derives.

```rust
use serde::{Deserialize, Serialize};

// ── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Surface {
    Hard,
    Clay,
    Grass,
    Carpet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TourneyLevel {
    GrandSlam,   // "G"
    Masters,     // "M"
    Tour,        // "A"  (ATP 250 / 500)
    Finals,      // "F"
    Olympics,    // "O"
    DavisCup,    // "D"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Round {
    R128,
    R64,
    R32,
    R16,
    Quarterfinal,
    Semifinal,
    Final,
    RoundRobin,  // "RR" — ATP Finals group stage, Olympics
    Bronze,      // "BR" — Olympics bronze medal match
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchOutcome {
    Completed,
    Retirement,
    Walkover,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImportStatus {
    Pending,
    Transformed,
    Failed(String),
}

// ── Score sub-types ───────────────────────────────────────────────────────────

/// One set in a match. tiebreak holds the loser's points if a tiebreak was
/// played, e.g. "7-6(4)" → winner_games=7, loser_games=6, tiebreak=Some(4).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Set {
    pub winner_games: u8,
    pub loser_games: u8,
    pub tiebreak: Option<u8>,
}

// ── Source document (raw CSV row from Supabase raw.matches.data) ─────────────
//
// Every field is String or Option<String> — no coercion, exactly as it
// arrives from the CSV. serde(default) fills in None for missing JSON keys.

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct RawMatch {
    pub tourney_id: String,
    pub tourney_name: String,
    pub surface: String,
    pub draw_size: String,
    pub tourney_level: String,
    pub tourney_date: String,   // "20240101"
    pub match_num: String,
    pub winner_id: String,
    pub winner_seed: String,    // "" when unseeded
    pub winner_name: String,
    pub winner_hand: String,
    pub winner_ht: String,
    pub winner_ioc: String,
    pub winner_age: String,
    pub loser_id: String,
    pub loser_seed: String,
    pub loser_name: String,
    pub loser_hand: String,
    pub loser_ht: String,
    pub loser_ioc: String,
    pub loser_age: String,
    pub score: String,          // "6-3 7-6(4) 6-4" | "W/O" | "6-4 3-0 RET"
    pub best_of: String,
    pub round: String,
    pub minutes: String,        // "" when missing
    pub w_ace: String,
    pub l_ace: String,
    pub winner_rank: String,
    pub loser_rank: String,
    pub winner_rank_points: String,
    pub loser_rank_points: String,
}

// Required by #[serde(default)] — all fields default to empty string
impl Default for RawMatch {
    fn default() -> Self {
        Self {
            tourney_id: String::new(),
            tourney_name: String::new(),
            surface: String::new(),
            draw_size: String::new(),
            tourney_level: String::new(),
            tourney_date: String::new(),
            match_num: String::new(),
            winner_id: String::new(),
            winner_seed: String::new(),
            winner_name: String::new(),
            winner_hand: String::new(),
            winner_ht: String::new(),
            winner_ioc: String::new(),
            winner_age: String::new(),
            loser_id: String::new(),
            loser_seed: String::new(),
            loser_name: String::new(),
            loser_hand: String::new(),
            loser_ht: String::new(),
            loser_ioc: String::new(),
            loser_age: String::new(),
            score: String::new(),
            best_of: String::new(),
            round: String::new(),
            minutes: String::new(),
            w_ace: String::new(),
            l_ace: String::new(),
            winner_rank: String::new(),
            loser_rank: String::new(),
            winner_rank_points: String::new(),
            loser_rank_points: String::new(),
        }
    }
}

// ── Normalized target record ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRecord {
    pub id: String,             // "{tourney_id}_{match_num}"
    pub tourney_id: String,
    pub tourney_name: String,
    pub surface: Surface,
    pub tourney_level: TourneyLevel,
    pub match_date: String,     // kept as "YYYY-MM-DD" string for now
    pub winner_name: String,
    pub loser_name: String,
    pub score_raw: String,      // original string preserved
    pub sets: Vec<Set>,
    pub outcome: MatchOutcome,
    pub round: Round,
    pub winner_seed: Option<u32>,
    pub loser_seed: Option<u32>,
    pub duration_minutes: Option<u32>,
    pub winner_aces: Option<u32>,
    pub loser_aces: Option<u32>,
    pub winner_rank: Option<u32>,
    pub loser_rank: Option<u32>,
    pub import_status: ImportStatus,
}
```

**Why `String` for `match_date`?** Avoids pulling in `chrono` before you need
it. Once all the other transforms pass tests, swap to `chrono::NaiveDate` as
a deliberate step — the change is isolated to `types.rs` and `transform.rs`.

---

## Step 4 — `src/error.rs`

```rust
#[derive(Debug, thiserror::Error)]
pub enum TransformError {
    #[error("unknown surface: {0}")]
    UnknownSurface(String),

    #[error("unknown round: {0}")]
    UnknownRound(String),

    #[error("unknown tournament level: {0}")]
    UnknownTourneyLevel(String),

    #[error("invalid date format '{0}' (expected YYYYMMDD)")]
    InvalidDate(String),

    #[error("invalid score: {0}")]
    InvalidScore(String),

    #[error("missing required field: {0}")]
    MissingField(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("record not found: {0}")]
    NotFound(String),
}
```

**`thiserror` practice point:** the `#[error("...")]` attribute generates the
`Display` impl for you. `{0}` refers to the first tuple field.
You get `Error` trait impl for free. Compare to doing this manually.

---

## Step 5 — `src/transform.rs`

Signature-first: write all the `fn` stubs, make them compile with `todo!()`,
then fill in one at a time. Tests go in `#[cfg(test)]` at the bottom of
this file.

```rust
use crate::error::TransformError;
use crate::types::{
    MatchOutcome, MatchRecord, RawMatch, Round, Set, Surface, TourneyLevel, ImportStatus,
};

// ── Public entry point ────────────────────────────────────────────────────────

pub fn transform(raw: RawMatch) -> Result<MatchRecord, TransformError> {
    todo!()
}

// ── Parsers ───────────────────────────────────────────────────────────────────

fn parse_surface(s: &str) -> Result<Surface, TransformError> {
    todo!()
}

fn parse_tourney_level(s: &str) -> Result<TourneyLevel, TransformError> {
    todo!()
}

fn parse_round(s: &str) -> Result<Round, TransformError> {
    todo!()
}

/// "20240101" → "2024-01-01"
/// Returns TransformError::InvalidDate if the string isn't 8 ASCII digits.
fn parse_date(s: &str) -> Result<String, TransformError> {
    todo!()
}

/// Parse score string into (sets, outcome).
/// Handles: "6-3 7-6(4) 6-4", "W/O", "6-4 3-0 RET", "6-4 6-7(6) 2-0 RET"
pub fn parse_score(s: &str) -> Result<(Vec<Set>, MatchOutcome), TransformError> {
    todo!()
}

/// "" or non-numeric → None; otherwise parse to u32.
fn parse_optional_u32(s: &str) -> Option<u32> {
    todo!()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // -- parse_surface --------------------------------------------------------

    #[test]
    fn surface_hard() {
        assert_eq!(parse_surface("Hard").unwrap(), Surface::Hard);
    }

    #[test]
    fn surface_unknown_errors() {
        assert!(parse_surface("Astroturf").is_err());
    }

    // -- parse_round ----------------------------------------------------------

    #[test]
    fn round_qf() {
        assert_eq!(parse_round("QF").unwrap(), Round::Quarterfinal);
    }

    #[test]
    fn round_rr() {
        assert_eq!(parse_round("RR").unwrap(), Round::RoundRobin);
    }

    // -- parse_date -----------------------------------------------------------

    #[test]
    fn date_valid() {
        assert_eq!(parse_date("20240101").unwrap(), "2024-01-01");
    }

    #[test]
    fn date_invalid_errors() {
        assert!(parse_date("Jan 1 2024").is_err());
    }

    // -- parse_score ----------------------------------------------------------

    #[test]
    fn score_straight_sets() {
        let (sets, outcome) = parse_score("6-3 6-4").unwrap();
        assert_eq!(sets.len(), 2);
        assert_eq!(outcome, MatchOutcome::Completed);
        assert_eq!(sets[0].winner_games, 6);
        assert_eq!(sets[0].loser_games, 3);
        assert_eq!(sets[0].tiebreak, None);
    }

    #[test]
    fn score_with_tiebreak() {
        let (sets, outcome) = parse_score("7-6(4) 6-3").unwrap();
        assert_eq!(outcome, MatchOutcome::Completed);
        assert_eq!(sets[0].tiebreak, Some(4));
    }

    #[test]
    fn score_retirement() {
        let (sets, outcome) = parse_score("6-4 6-7(4) 0-0 RET").unwrap();
        assert_eq!(outcome, MatchOutcome::Retirement);
        // incomplete last set is still recorded
        assert_eq!(sets.len(), 3);
    }

    #[test]
    fn score_walkover() {
        let (_sets, outcome) = parse_score("W/O").unwrap();
        assert_eq!(outcome, MatchOutcome::Walkover);
    }

    // -- parse_optional_u32 ---------------------------------------------------

    #[test]
    fn optional_u32_numeric() {
        assert_eq!(parse_optional_u32("14"), Some(14));
    }

    #[test]
    fn optional_u32_empty() {
        assert_eq!(parse_optional_u32(""), None);
    }
}
```

**The order to implement in:** `parse_optional_u32` → `parse_surface` →
`parse_tourney_level` → `parse_round` → `parse_date` → `parse_score` →
`transform`. Each one builds on the last.
`parse_score` is the most interesting — start with the `W/O` case, then
straight sets, then tiebreak, then retirement.

---

## Step 6 — `src/repository.rs`

```rust
use std::collections::HashMap;

use crate::error::RepositoryError;
use crate::types::MatchRecord;

// ── Trait ────────────────────────────────────────────────────────────────────

pub trait MatchRepository {
    fn save(&mut self, record: MatchRecord) -> Result<(), RepositoryError>;
    fn get(&self, id: &str) -> Result<&MatchRecord, RepositoryError>;
    fn list(&self) -> Vec<&MatchRecord>;
    fn count(&self) -> usize;
}

// ── In-memory implementation ─────────────────────────────────────────────────

pub struct InMemoryMatchRepository {
    records: HashMap<String, MatchRecord>,
}

impl InMemoryMatchRepository {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }
}

impl MatchRepository for InMemoryMatchRepository {
    fn save(&mut self, record: MatchRecord) -> Result<(), RepositoryError> {
        todo!()
    }

    fn get(&self, id: &str) -> Result<&MatchRecord, RepositoryError> {
        todo!()
    }

    fn list(&self) -> Vec<&MatchRecord> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn sample_record(id: &str) -> MatchRecord {
        MatchRecord {
            id: id.to_string(),
            tourney_id: "2024-0001".to_string(),
            tourney_name: "Brisbane".to_string(),
            surface: Surface::Hard,
            tourney_level: TourneyLevel::Tour,
            match_date: "2024-01-01".to_string(),
            winner_name: "Dimitrov G.".to_string(),
            loser_name: "Rune H.".to_string(),
            score_raw: "7-6(5) 6-4".to_string(),
            sets: vec![
                Set { winner_games: 7, loser_games: 6, tiebreak: Some(5) },
                Set { winner_games: 6, loser_games: 4, tiebreak: None },
            ],
            outcome: MatchOutcome::Completed,
            round: Round::Final,
            winner_seed: Some(2),
            loser_seed: Some(1),
            duration_minutes: Some(136),
            winner_aces: Some(8),
            loser_aces: Some(9),
            winner_rank: Some(14),
            loser_rank: Some(8),
            import_status: ImportStatus::Transformed,
        }
    }

    #[test]
    fn save_and_get_roundtrip() {
        let mut repo = InMemoryMatchRepository::new();
        let record = sample_record("test-001");
        repo.save(record.clone()).unwrap();
        let fetched = repo.get("test-001").unwrap();
        assert_eq!(fetched.id, "test-001");
        assert_eq!(fetched.winner_name, "Dimitrov G.");
    }

    #[test]
    fn get_missing_returns_not_found() {
        let repo = InMemoryMatchRepository::new();
        let err = repo.get("nope").unwrap_err();
        assert!(matches!(err, RepositoryError::NotFound(_)));
    }

    #[test]
    fn count_tracks_inserts() {
        let mut repo = InMemoryMatchRepository::new();
        assert_eq!(repo.count(), 0);
        repo.save(sample_record("a")).unwrap();
        repo.save(sample_record("b")).unwrap();
        assert_eq!(repo.count(), 2);
    }

    #[test]
    fn list_returns_all() {
        let mut repo = InMemoryMatchRepository::new();
        repo.save(sample_record("a")).unwrap();
        repo.save(sample_record("b")).unwrap();
        assert_eq!(repo.list().len(), 2);
    }
}
```

---

## Order of work

1. `Cargo.toml` — add deps, run `cargo build` to confirm workspace compiles
2. `lib.rs` — four `pub mod` lines
3. `types.rs` — all structs/enums, no logic
4. `error.rs` — two error enums
5. `transform.rs` — stubs first (`todo!()`), then implement one fn at a time,
   running `cargo test` after each
6. `repository.rs` — stub then implement; tests should pass once `save`/`get`
   are filled in

`cargo test` at any stage will show you which `todo!()`s are still pending
(they panic with "not yet implemented"). That's your progress tracker.
