# ADR 0001: Himalayas as first job source

## Status
Accepted

## Context
Phase 2 needs one compliant job source behind a `Source` adapter. Scraping and LinkedIn/Indeed are out of policy. The CLI should sync occasionally, store jobs locally, and rank a 24h window for `today`.

## Decision
Use the **Himalayas** public Remote Jobs API as the first adapter.

- Browse: `GET https://himalayas.app/jobs/api?limit=20`
- Docs: https://himalayas.app/docs/remote-jobs-api
- Attribution: keep `applicationLink` (includes their utm), `source = "himalayas"`
- Do not redistribute listings to other aggregators

## Consequences
- Official JSON API; no HTML scrape
- Himalayas cache refreshes about daily — poll sparingly (a few times/day is enough)
- Optional free API key later for higher limits; Phase 2 uses unauthenticated requests with a clear User-Agent
- Our freshness window (24h from `pubDate`) may yield few rows if their `pubDate` skews older than a day; `today` still filters tech + fresh correctly
- Tech filter remains local title-keyword rules (`Job::is_tech`), not Himalayas categories alone

## Alternatives considered
- Remotive — also compliant; deferred as second source
- Remote OK — public JSON; deferred
- mobile.career — no public API
- Adzuna/Jooble — broader aggregators; more noise and keys
