use std::collections::HashMap;
use std::hash::Hash;

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

    fn get(&self, id: &str) -> Result<MatchRecord, RepositoryError>{
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

    fn sample_record
}