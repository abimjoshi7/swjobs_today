# swjobs_today

CLI for **software jobs for today**: tech-only roles from compliant sources, short freshness window, ranked by fit, with a reusable answer brain for common application fields.

Auto-apply and scraping of non-API boards are **not** the product.

## Docs

- **[docs/roadmap.md](docs/roadmap.md)** — intentions, requirements, and phased plan

## Quick start

```bash
# set skills (stored in .data/profile.json)
cargo run -- profile set --skills rust,postgresql,aws

# ranked fixture feed for today
cargo run -- today
cargo run -- today --limit 5

cargo run -- profile show
```

Binary name: `sjt` (`cargo run --bin sjt -- today`).

## Data

- Profile: `.data/profile.json` (gitignored)
- Jobs: bundled `src/fixtures/jobs.json` (relative `age_hours` from “now”)

## Status

Phase 1 — profile + fixture ranked `today`. See [docs/roadmap.md](docs/roadmap.md) for next milestones.
