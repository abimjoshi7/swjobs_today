# swjobs_today

CLI for **software jobs for today**: tech-only roles from compliant sources, short freshness window, ranked by fit, with a reusable answer brain for common application fields.

Auto-apply and scraping of non-API boards are **not** the product.

## Docs

- **[docs/roadmap.md](docs/roadmap.md)** — intentions, requirements, and phased plan
- **[docs/decisions/0001-himalayas-source.md](docs/decisions/0001-himalayas-source.md)** — why Himalayas as first source

## Quick start

```bash
# set skills (stored in .data/profile.json)
cargo run -- profile set --skills rust,postgresql,aws

# live sync from Himalayas (network; keep to a few times per day)
cargo run -- sync

# ranked list from .data/jobs.json (tech + last 24h)
cargo run -- today
cargo run -- today --limit 5

cargo run -- profile show
```

Binary name: `sjt` (`cargo run --bin sjt -- today`).

### Manual live sync

```bash
cargo run -- sync
```

- Hits `GET https://himalayas.app/jobs/api?limit=20`
- Writes fresh rows to `.data/jobs.json`, sweeps >24h
- Attribution: job URLs keep Himalayas `applicationLink`
- Respect rate limits / daily cache — do not hammer the API
- Offline tests use `src/fixtures/himalayas_*.json` (no network in `cargo test`)

## Data

- Profile: `.data/profile.json` (gitignored)
- Synced jobs: `.data/jobs.json` (gitignored)
- Bundled fixtures: `src/fixtures/` (adapter + ranking tests)

## Status

Phase 2 — Himalayas `sync` + stored `today`. See [docs/roadmap.md](docs/roadmap.md) for next milestones.
