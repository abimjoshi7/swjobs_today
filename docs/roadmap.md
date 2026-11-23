# Roadmap — swjobs_today

CLI for **software jobs you can act on today**: compliant sources only, tech roles only, short freshness window, ranked by fit — plus a reusable answer brain for common application fields. Auto-apply and vision/OCR are **out of scope** for the core product.

---

## Intentions

### Why this exists

Job search noise is the problem — stale listings, low-trust boards, and repeating the same form answers. Spreadsheet-style trackers add stress without leverage. This tool should **surface today’s worth-applying tech roles** and **remember answers** so each application gets faster.

### Product principles

1. **Clarity over cleverness** — name and UX say what it does (`swjobs_today`).
2. **Compliance-first sources** — only platforms with official APIs/MCPs (or explicit partner terms). No LinkedIn/Indeed scraping; no ToS cat-and-mouse as a feature.
3. **Tech roles only** — title/keyword/stack filters keep the feed relevant and the DB small.
4. **Freshness as USP** — prefer listings within a **24h** window (configurable later). Expired rows are dropped or ignored so the tool rewards daily use and stays lean.
5. **Rank, don’t nag** — score by fit × freshness × source trust. No guilt CRM as the main loop.
6. **Apply assist, not guaranteed auto-apply** — map common form fields to a profile; ask only for gaps; save patterns. Submit only where a provider explicitly allows it and the user confirms.
7. **Learning project with real use** — ship vertical slices you run yourself; deepen engineering habits (types, tests, adapters, docs) without boiling the ocean.

### Non-goals (v1–v2)

- Mass auto-apply / stealth browsers / CAPTCHA bypass
- Screenshot/vision as the primary form reader
- Full multi-ATS SPA automation on day one
- Becoming a generic “dealflow” or non-tech job board

---

## Requirements

### Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| F1 | Maintain a **user profile** (identity, links, stack, work auth, résumé path, freeform notes) on disk | P0 |
| F2 | **Ingest** jobs from ≥1 compliant source behind a clear adapter boundary | P0 |
| F3 | **Filter** to tech/software roles; reject or down-rank off-domain | P0 |
| F4 | Enforce **freshness** (default: hide/drop after 24h from `posted_at`, else ingest time) | P0 |
| F5 | **Rank** today’s list (skills/keywords overlap + recency; source trust when available) | P0 |
| F6 | CLI command to show **today’s** top N roles (title, company, link, score, age) | P0 |
| F7 | **Answer brain**: canonical fields + map of custom Q→A; prompt on unknown; reuse on similar labels | P1 |
| F8 | Produce an **apply packet** (filled fields / text dump) for manual or assisted submit | P1 |
| F9 | **Source registry** with rate-limit/throttle config and basic trust weight | P2 |
| F10 | Optional **MCP/tool** façade over the same operations (search/today/profile) | P3 |
| F11 | Optional true **API apply** adapter where a provider supports it — never implied for all sources | P3 |

### Non-functional

| ID | Requirement |
|----|-------------|
| N1 | Rust binary crate; clear modules; prefer `Result` + typed errors over panics in library logic |
| N2 | Deterministic tests for ranking, freshness, and tech filter using **fixture JSON** (no live network in CI) |
| N3 | Config + data under a local app dir (e.g. `~/.swjobs_today/` or project `.data/`) — document the choice |
| N4 | Respect published rate limits; debounce/throttle are features, not afterthoughts |
| N5 | Document source ToS/constraints when an adapter is added (short note or ADR) |
| N6 | Keep the dream backlog visible; do not block P0 on P2+ |

### Constraints

- Language: Rust (edition as in `Cargo.toml`)
- Interface: CLI first (`clap` or equivalent when ready)
- Storage: start with JSON files; SQLite only if pain appears
- Sources: allowlist only; LinkedIn/Indeed explicitly out of policy

---

## Plan (phases)

### Phase 0 — Skeleton you can run

**Outcome:** `cargo run` prints help / a placeholder `today` message; repo has this roadmap.

- [x] `cargo new swjobs_today`
- [x] Minimal CLI entry (`today`, `version` / `--help`)
- [ ] Root README pointing at docs

### Phase 1 — Profile + fake ranked feed

**Outcome:** You can set profile skills and see a ranked list from **hardcoded/fixture** jobs (no network).

- [ ] `Profile` model + load/save JSON
- [ ] `Job` model (`id`, source, title, company, url, posted_at, tags/skills)
- [ ] Tech filter + freshness check + score function
- [ ] Commands: `profile show|set`, `today`
- [ ] Unit tests for score / filter / expiry

### Phase 2 — First real source

**Outcome:** `sync` pulls from one compliant API; `today` uses stored jobs within 24h.

- [ ] `Source` trait + one adapter
- [ ] Persist synced jobs; sweeper ignores/deletes expired
- [ ] Fixture-based adapter tests; manual live sync documented
- [ ] Short ADR: why this source, rate limits, tech filter rules

### Phase 3 — Answer brain

**Outcome:** Repeated application questions need fewer keystrokes.

- [ ] Canonical profile fields + `custom_answers`
- [ ] `answer` / `ask` (or interactive fill) with simple label normalization
- [ ] `packet <job_id>` prints values to paste into a form
- [ ] No browser automation required

### Phase 4 — Engineering hardening

**Outcome:** Looks like software you’d show in an interview, not only a script.

- [ ] Split `lib` + `bin` if main grows
- [ ] Structured errors, basic logging
- [ ] Integration test: fixture sync → stable `today` output
- [ ] `learnings.md` / ADRs as decisions appear

### Phase 5 — Stretch (only after daily use)

- Second source + trust weights  
- Throttle dashboard / apply queue (assisted)  
- MCP wrapper  
- One ATS-assisted path (AX/DOM) — still not vision-first

---

## Success criteria

**Personal:** You open `swjobs_today today` on real job-search days and trust the list enough to click through.

**Engineering:** Adapters and ranking are tested without the network; adding a source does not rewrite the CLI.

**Product:** New users understand the tool in one sentence — *Software jobs for today, ranked for you, with answers you reuse.*

---

## Backlog (explicitly deferred)

- Vision/screenshot form reading  
- Guaranteed auto-apply / mass submit  
- LinkedIn, Indeed, and any non-compliant scrape  
- Mobile app / heavy UI  
- Multi-tenant SaaS and growth metrics before solo daily use

---

## Related docs

| Path | Purpose |
|------|---------|
| [README.md](README.md) | Docs index |
| `../README.md` | Repo entry (when present) |
| `decisions/` | ADRs when architectural choices stick |
| `learnings.md` | Working notes (add when useful) |
