use crate::error::TransformError;
use crate::types::{
    MatchOutcome, MatchRecord, RawMatch, Round, Set, Surface, TourneyLevel, ImportStatus,
};

// ── Public entry point ────────────────────────────────────────────────────────
pub fn transform(raw: RawMatch) -> Result<Surface, TransformError>{
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
        assert!(parse_surface("Astrotruf").is_err());
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
        let (sets,outcome) = parse_score("7-6(4) 6-3").unwrap();
        assert_eq!(outcome, MatchOutcome::Completed);
        assert_eq!(sets[0].tiebreak, Some(4));
    }

    #[test]
    fn score_retirement() {
        let (sets, outcome) = parse_score("6-4 6-7(4) 0-0 RET").unwrap();
        assert_eq!(outcome, MatchOutcome::Retirement);
        //incomplete last set is still recorded
        assert_eq!(sets.len(), 3);
    }

    #[test]
    fn score_walkover() {
        let (_sets, outcome) = parse_score("W/O").unwrap();
        assert_eq!(outcome, MatchOutcome::Walkover);
    }

    #[test]
    fn optional_u32_empty() {
        assert_eq!(parse_optional_u32(""), None);
    }

    

}
