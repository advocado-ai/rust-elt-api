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
    pub tourney_name: String,
    pub surface: String,
    pub draw_size: String,
    pub tourney_level: String,
    pub match_num: String,
    pub winner_id: String,
    pub winner_seed: String,
    pub winner_name: String,
    pub winnder_hand: String,
    pub winner_ht: String,
    pub winner_ioc: String,
    pub winnder_age: String,
    pub loser_id: String,
    pub loser_name: String,
    pub loser_hand: String,
    pub loser_ht: String,
    pub loser_ioc: String,
    pub loser_age: String,
    pub score: String,
    pub best_of: String,
    pub round: String,
    pub minutes: String,
    pub w_ace: String,
    pub l_ace: String,
    pub winner_rank: String,
    pub loser_rank: String,
    pub winner_rank_points: String,
    pub loser_rank_points: String,    
}

impl Default for RawMatch {
    fn default() -> Self {
        Self {
            tourney_id: String::new(),
            tourney_name: String::new(),
            surface:String::new(),
            draw_size:String::new(),
            tourney_level: String::new(),
            match_num: String::new(),
            winner_id: String::new(),
            winner_seed: String::new(),
            winner_name: String::new(),
            winnder_hand: String::new(),
            winner_ht: String::new(),
            winner_ioc: String::new(),
            winnder_age: String::new(),
            loser_id: String::new(),
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
    pub id: String,
    pub tourney_id: String,
    pub tourney_name: String,
    pub surface: Surface,
    pub tourney_level: TourneyLevel,
    pub match_date: String,
    pub winner_name: String,
    pub loser_name: String,
    pub score_raw: String,
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
