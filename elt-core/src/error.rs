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

