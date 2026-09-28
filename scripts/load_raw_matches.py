"""
Load ATP match CSVs into Supabase raw.matches table.

Usage:
    conda run -n advocado-env python scripts/load_raw_matches.py
    conda run -n advocado-env python scripts/load_raw_matches.py --file _data/atp_matches_2023.csv
    conda run -n advocado-env python scripts/load_raw_matches.py --batch-size 500
"""
import argparse
import csv
import json
import os
import sys

import psycopg2
import psycopg2.extras
from dotenv import load_dotenv
from rich.console import Console
from rich.progress import (
    BarColumn,
    MofNCompleteColumn,
    Progress,
    SpinnerColumn,
    TaskProgressColumn,
    TextColumn,
    TimeElapsedColumn,
    TimeRemainingColumn,
)

load_dotenv()
console = Console()

DATABASE_URL = os.environ.get("DATABASE_URL")
if not DATABASE_URL:
    console.print("[red]DATABASE_URL not set[/red] — copy .env.example to .env and fill it in")
    sys.exit(1)

DDL = """
CREATE SCHEMA IF NOT EXISTS raw;
CREATE TABLE IF NOT EXISTS raw.matches (
    id          BIGSERIAL    PRIMARY KEY,
    source_file TEXT         NOT NULL,
    data        JSONB        NOT NULL,
    loaded_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS raw_matches_source_file_idx ON raw.matches (source_file);
"""


def connect():
    url = DATABASE_URL
    if "sslmode" not in url:
        url += "?sslmode=require"
    return psycopg2.connect(url, connect_timeout=15)


def load(csv_path: str, batch_size: int = 250) -> int:
    source_file = os.path.basename(csv_path)

    console.print(f"\n[bold]Reading[/bold] {csv_path} ...", end=" ")
    with open(csv_path, newline="", encoding="utf-8") as f:
        rows = list(csv.DictReader(f))
    console.print(f"[green]{len(rows):,} rows[/green]")

    console.print("[bold]Connecting[/bold] to Supabase ...", end=" ")
    conn = connect()
    console.print(f"[green]ok[/green] (server {conn.server_version})")

    cur = conn.cursor()

    console.print("[bold]Setting up schema[/bold] ...", end=" ")
    cur.execute(DDL)
    conn.commit()
    console.print("[green]done[/green]")

    cur.execute("SELECT COUNT(*) FROM raw.matches WHERE source_file = %s", (source_file,))
    existing = cur.fetchone()[0]
    if existing:
        console.print(f"[yellow]{source_file}: {existing:,} rows already present — skipping[/yellow]")
        conn.close()
        return 0

    batches = [rows[i : i + batch_size] for i in range(0, len(rows), batch_size)]

    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        BarColumn(),
        MofNCompleteColumn(),
        TaskProgressColumn(),
        TimeElapsedColumn(),
        TimeRemainingColumn(),
        console=console,
    ) as progress:
        task = progress.add_task(f"Inserting {source_file}", total=len(rows))

        for batch in batches:
            values = [(source_file, json.dumps(row)) for row in batch]
            psycopg2.extras.execute_values(
                cur,
                "INSERT INTO raw.matches (source_file, data) VALUES %s",
                values,
            )
            conn.commit()
            progress.advance(task, len(batch))

    console.print(f"\n[bold green]✓ Inserted {len(rows):,} rows[/bold green] from {source_file}")
    conn.close()
    return len(rows)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--file", default="_data/atp_matches_2024.csv")
    parser.add_argument("--batch-size", type=int, default=250)
    args = parser.parse_args()

    if not os.path.exists(args.file):
        console.print(f"[red]File not found:[/red] {args.file}")
        sys.exit(1)

    load(args.file, args.batch_size)
