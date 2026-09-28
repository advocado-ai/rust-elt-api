use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Surface {
    Hard,
    Clay,
    Grass,
    Carpet,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TourneyLevel {
    Grandslam,
    Masters,
    Tour,
    Finals,
    Olympics,
    DavisCup,
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
    RoundRobin,
    Bronze,
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
    pub tiebreak: Option<u8>
}

// ── Source document (raw CSV row from Supabase raw.matches.data) ─────────────
//
// Every field is String or Option<String> — no coercion, exactly as it
// arrives from the CSV. serde(default) fills in None for missing JSON keys.

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct RawMatch {
    pub tourney_id: String,
    
}

