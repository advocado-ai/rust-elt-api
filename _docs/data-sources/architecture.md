# Data Architecture

## Source data

**Jeff Sackmann tennis datasets** — use the original repos directly:
- ATP: `https://github.com/JeffSackmann/tennis_atp`
- WTA: `https://github.com/JeffSackmann/tennis_wta`
- Slam point-by-point: `https://github.com/JeffSackmann/tennis_slam_pointbypoint`
- License: CC BY-NC-SA 4.0 (non-commercial, share-alike)

> **Do not use the Aneeshers/tennis-sackmann-archive mirror** on HuggingFace (797 MB,
> 4 likes). HuggingFace free public storage is "best-effort" — low-engagement datasets
> are the first to be pruned. Go to the upstream repos.
>
> **Do not clone the full archive next to this project.** Download only the files you need.

| Folder | Contents | Coverage |
|--------|----------|----------|
| `atp/` | Match results, rankings, player tables | Through 2026 |
| `wta/` | Match results, rankings, player tables | Through 2026 |
| `slam_pointbypoint/` | Point-by-point logs for 4 Grand Slams | 2011–2024 |

For this project we start with **ATP match results** (`atp_matches_YYYY.csv`).
One year (~2,500 rows) is enough to cover the transform logic and keep storage trivial.

### Acquiring the data

```bash
# One file, ~1-2 MB — run once from the project root
mkdir -p _data
curl -L \
  "https://raw.githubusercontent.com/JeffSackmann/tennis_atp/master/atp_matches_2024.csv" \
  -o _data/atp_matches_2024.csv
```

Add `_data/` to `.gitignore` — raw data files don't belong in the repo.
Commit a small `_data/fixtures/` subfolder with 5–10 hand-picked rows for unit
tests so tests run offline without the full CSV.

---

## ELT boundary

```
Sackmann CSVs (GitHub / HuggingFace)
    │
    │  Extract  (future: a small script / curl — not the Rust API's job)
    ▼
Raw layer  ─────────────────────────────────────────────────────────────
  Rows land as flat/JSONB records, mirroring the CSV shape exactly.
  No normalization yet. This is the "Mongo-like source document" the
  project roadmap refers to.
    │
    │  Transform + Load  (Rust API: POST /imports)
    ▼
Normalized layer ────────────────────────────────────────────────────────
  MatchRecord rows: typed enums, parsed scores, coerced ranks, dates.
  The repository trait abstracts the storage backend.
```

The Rust API owns only the **T + L** steps. The extract step is out of scope
for Part 1 and early Part 2 — raw documents are supplied as hardcoded fixtures
in tests, then later POSTed via curl or a loader script.

---

## Storage options

### Raw layer (landing zone)

| Option | Free tier | Notes |
|--------|-----------|-------|
| Supabase Storage | 1 GB | Object storage (S3-compatible); CSVs sit as files |
| Supabase Postgres (`raw` schema, `JSONB` column) | 500 MB | Best fit — rows look like Mongo documents; sqlx reads them natively |
| Cloudflare R2 | 10 GB | Best free object storage if volume grows; S3-compatible |
| HuggingFace directly | Unlimited (public) | Just reference the archive; no ingestion needed for learning work |

**Chosen: Supabase Postgres `raw` schema** — one `raw_matches` table with a
`JSONB` column. Each row is one CSV row serialized to JSON. This gives the
most realistic "Mongo-like document" feel and keeps everything in one place.

### Normalized layer (ELT destination)

| Option | Free tier | Notes |
|--------|-----------|-------|
| SQLite (local file) | Free | Zero friction; perfect for dev and CI; sqlx supports natively |
| Supabase Postgres | 500 MB | Same instance as raw layer; user familiar with it; sqlx natively supported |
| Neon.tech (serverless Postgres) | Free | Auto-pauses; good alternative if Supabase free tier fills up |
| BigQuery | 10 GB storage + 1 TB queries/month | Best for analytics; user already knows it; NOT natively supported by sqlx — needs REST client or a separate loader |
| DuckDB / MotherDuck | Free tier | Excellent for ad-hoc analytics queries over the normalized layer; not a sqlx target |

**Chosen (phased):**
- **Part 1 + early Part 2**: in-memory `HashMap` (no database at all — per roadmap)
- **Part 2 mid**: SQLite via sqlx (easiest swap, no server needed)
- **Part 2 late / demo**: Supabase Postgres via sqlx (same trait, different impl)
- **Optional analytics layer**: BigQuery or DuckDB for querying the normalized records outside of Rust

---

## Recommended full-stack layout

```
HuggingFace / GitHub CSVs
    │  (one-time load script — Python or curl)
    ▼
Supabase
  schema: raw
    table: raw_matches  (id BIGSERIAL, data JSONB, loaded_at TIMESTAMPTZ)
    ─────────────────────────────────────────────────────────────────
  schema: public
    table: match_records (id TEXT PK, tournament TEXT, surface TEXT,
                          round TEXT, match_date DATE, winner TEXT,
                          loser TEXT, score JSONB, winner_rank INT4,
                          loser_rank INT4, duration_minutes INT4,
                          import_status TEXT, created_at TIMESTAMPTZ)
```

The Rust `ImportRepository` trait writes to `match_records`.
A future `RawRepository` or loader script writes to `raw_matches`.

---

## Cost estimate

| Component | Provider | Tier | Monthly cost |
|-----------|----------|------|--------------|
| Raw + normalized storage | Supabase | Free | $0 |
| Analytics queries | BigQuery | Free (≤1 TB/mo) | $0 |
| Overflow object storage | Cloudflare R2 | Free (≤10 GB) | $0 |
| Supabase Pro (if free pauses) | Supabase | Pro | $25 |

Realistically: **$0** during development, **$25/mo max** if you need the
Supabase instance to stay awake for a demo.

---

## Open questions

- [ ] Do we want point-by-point data (`slam_pointbypoint`) in scope, or just
      match-level results for now?
- [ ] Should the `score` column in `match_records` be structured JSONB
      (array of sets) or a raw string stored for later parsing?
- [ ] Extract step: Python script, `curl` + jq, or a separate Rust CLI?
