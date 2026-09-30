use std::collections::HashMap;
use std::hash::Hash;
use std::vec;

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
        self.records.insert(record.id.clone(), record);
        Ok(())
    }

    fn get(&self, id: &str) -> Result<&MatchRecord, RepositoryError> {
        let m_record = match self.records.get(id){
            Some(matchrecord) => Ok(matchrecord),
            None => Err(RepositoryError::NotFound(format!("no matchrecord for id: {id}"))),
        };

        m_record

        
    }

    fn list(&self) -> Vec<&MatchRecord> {
        let mut vec_of_m_records = Vec::<&MatchRecord>::new();

        for (_,v) in &self.records{
            vec_of_m_records.push(v);
        }

        vec_of_m_records

    }

    fn count(&self) -> usize {
        self.records.len()
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
                Set { winner_games: 7, loser_games: 6, tiebreak: Some(5)},
                Set { winner_games: 6, loser_games: 4, tiebreak: None},
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
