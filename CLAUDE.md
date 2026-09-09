# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

iptmnet_client is a Rust CLI tool that provides an interface to the IPTMNet API for querying post-translational modification (PTM) data about enzymes, substrates, and proteins. The project is licensed under GPL-3.0-or-later.

## Project Structure

```text
iptmnet_client/
├── src/
│   ├── iptmlib/          # Library crate for API data models
│   │   ├── lib.rs        # Core library with API response types
│   │   ├── models/       # Data models for API responses
│   │   └── mod.rs        # Module exports
│   └── iptmnet/          # CLI binary
│       ├── main.rs      # Entry point
│       ├── cli.rs       # Clap argument parsing
│       └── helpers.rs   # Helper types (ItemType, PtmType, Role)
├── apispec/
│   └── iptmnet.yaml      # OpenAPI specification for IPTMNet API
├── guide/                 # User documentation (mdBook)
├── scripts/              # Build/automation scripts
├── .github/workflows/    # CI configurations
└── .config/              # Project configuration
```

## Architecture

The project has a simple CLI + library architecture:

- **CLI Binary** (`src/iptmnet/`): Uses `clap` for argument parsing, `tokio` for async runtime, `reqwest` for HTTP requests to the IPTMNet API.
- **Library Crate** (`src/iptmlib/`): Provides typed data models for API responses including `Protein`, `Organism`, `ProteinVec`.
- **API Base URL**: Hardcoded to `https://research.bioinformatics.udel.edu/iptmnet/api/search`

Key design decisions:

- Error handling uses `anyhow` with `IPTMResultError` enum (DeserializingError, APIRequestError)
- API responses are represented as enum variants in `IPTMResult`
- Data models use `serde` for JSON serialization/deserialization

## Common Development Tasks

### Build

```bash
cargo build
```

### Run

```bash
cargo run -- search <QUERY>
```

### Run with full CLI options

```bash
cargo run -- --search <QUERY> --item-type <ITEM_TYPE> --ptm-type <PTM> --role <ROLE> --organism <ORGANISM>
```

### Format code

```bash
cargo +nightly fmt --all
```

### Lint with Clippy

```bash
cargo clippy -- -D warnings
```

### Run all tests

```bash
cargo test
```

### Run tests with nextest (faster)

```bash
cargo nextest run
```

### Run a single test

```bash
cargo test -- <TEST_NAME>
```

### Run tests with nextest for a specific test

```bash
cargo nextest run -- <TEST_NAME>
```

### Check formatting with nextest config

```bash
cargo +nightly fmt --all -- --check
```

### Run the application directly

```bash
cargo run --release -- --help
```

### Run with nextest showing test status

```bash
cargo nextest run --status-level pass
```

## CI/CD Workflow

The CI workflow (`.github/workflows/ci.yaml`) runs on:

- **Platforms**: Ubuntu, macOS, Windows (latest)
- **Rust toolchains**: stable, beta, nightly, MSRV (1.86.0)

On push or PR merge, it:

1. Builds the project
2. Runs tests
3. Checks formatting
4. Runs clippy linting
5. Updates GitHub badges for status reporting
6. Optionally generates code coverage (to Codecov)
7. Optionally builds Docker images (for doc changes)
8. Optionally bumps version and releases (for successful PR merges)

## Testing Strategy

- Unit tests are written with standard `#[test]` attributes
- The project uses `rstest` for async test fixtures
- Code coverage is generated using `cargo-llvm-cov`
- Doctests are included in coverage reports
- The `nextest.toml` config sets:
  - `retries = 0` (no retries in CI)
  - `test-threads = "num-cpus"`
  - `slow-timeout = { period = "60s", terminate-after = 2 }`
  - `leak-timeout = "100ms"`

## API Parameters

The IPTMNet API supports these query parameters:

- `search_term` (required): The term to search for
- `term_type` (required): "all", "uniprot-id", "protein-gene-name", or "pmid"
- `role` (required): "enzyme", "substrate", or "both"/"either"
- `ptm_type` (optional): Array of PTM types (phosphorylation, acetylation, ubiquitination, etc.)
- `organism` (optional): Taxon IDs to filter by
- `start_index`, `end_index` (optional): Pagination controls
- `paginate` (optional): "true" or "false"

Supported PTM types include: Acetylation, C-Glycosylation, Myristoylation, Ubiquitination, N-Glycosylation, S-Glycosylation, Phosphorylation, S-Nitrosylation, O-Glycosylation, Methylation, Sumoylation

## Code Style

- Use `rustfmt` for formatting (config: `.rustfmt.toml`)
- Run `cargo clippy -- -D warnings` before committing
- Follow conventional commits for commit messages (used by pre-commit hooks)
- License scanning is configured in `about.toml`

## Security Considerations

- The project uses `detect-secrets` pre-commit hook with baseline `.secrets.baseline`
- No hardcoded secrets should be committed
- Secrets should use environment variables

## License

This project is licensed under GPL-3.0-or-later. See `LICENSE` file for details.

## gstack

Use the **/browse** skill (from gstack) for all web browsing tasks.

### Never Use

- Never use `mcp__claude-in-chrome__*` tools — always use `/browse` instead.

### Available Skills

Use these skills for planning, design, testing, and development tasks:

- **/office-hours** — Brainstorming, idea exploration, startup thinking
- **/plan-ceo-review** — CEO/founder-mode strategic review, scope expansion
- **/plan-eng-review** — Engineering review, architecture, data flow, edge cases
- **/plan-design-review** — Design critique, visual audits, layout review
- **/design-consultation** — Design system creation, brand guidelines
- **/review** — Pre-landing PR review, code review
- **/ship** — Ship workflow: merge, CI, deploy, verify
- **/browse** — Headless browser for QA testing and site dogfooding
- **/qa** — Systematic QA testing, test and fix bugs
- **/qa-only** — QA report mode: test without fixing
- **/design-review** — Visual QA, design polish, fix visual issues
- **/setup-browser-cookies** — Import cookies for authenticated testing
- **/retro** — Engineering retrospective, commit history analysis
- **/investigate** — Systematic debugging with root cause analysis
- **/document-release** — Update documentation post-ship
- **/codex** — Code review, challenge mode, second opinion
- **/careful** — Safety mode: destructive command warnings
- **/freeze** — Restrict edits to specific directory
- **/guard** — Full safety: warnings + directory-scoped edits
- **/unfreeze** — Clear edit restrictions
- **/gstack-upgrade** — Upgrade gstack to latest version

### Example Usage

```bash
# To view a website:
/browse

# To test a feature on a live site:
/browse

# For authentication before testing:
/setup-browser-cookies
```
