# Changelog - Vine

Vine is our gRPC protocol definition - the `.proto` contract spoken between
Mountain and the Cocoon/Air sidecars. The actual `Vine.proto` lives in
Mountain's tree at `Element/Mountain/Proto/`; this repository carries the
documentation surface and version history. This file records what we built in
our voice, version by version. Format adapted from
[Keep a Changelog](https://keepachangelog.com/).

## [v2.1] - Full Workbench Lift (April 2026)

We brought Vine's documentation surface in line with the rest of the v2.1 wave -
the protocol stayed binary-stable through this window; the changes here are all
in the docs.

### Added

- **CHANGELOG following Keep a Changelog format** (`5a1f90f`, 2026-04-17).
- **Comprehensive architecture documentation** in the README (`e1f7b8a`,
  2026-04-05) with benefit-focused rewrite passes (`1b9ebbe`, `6d3f208`,
  2026-04-04).
- **Rust icon** added to the README header (`5dd25ad`, 2026-04-04).
- **See Also** section linking to architecture overview and related Elements
  (`6698d9f`, 2026-04-04).

### Changed

- **GitHub URLs in README documentation** corrected (`b628623`, 2026-04-16).
- **Documentation formatting and table alignment** improved (`a0faf14`,
  2026-04-06).

## [v2.0] - Editor Launch (Q1 2026)

The protocol stayed stable through the editor-launch window. Three unlabelled
saves on 2026-01-21, 2026-02-20, 2026-03-19/21 carried proto-doc edits that
landed alongside Mountain's gRPC handler split

- but the wire format itself didn't break compatibility.

## [v1.x] - Pre-Documentation Scaffold (April 2025 - January 2026)

Vine existed as a placeholder repository through this window. Six total commits
across the year, all unlabelled scaffolding pushes (`4c1e9ac` 2025-04-16,
`b638cb5` 2025-06-02, `d278aad` 2025-06-08, `c9e7c4b` 2025-09-12, `e5180ec`
2025-09-26, `9f46e94` 2026-01-21). The actual `Vine.proto` lived in Mountain
throughout this period; this repository was set up as the eventual home for the
protocol's versioned documentation.

## [v0.0] - Project Inception (April 2025)

Repository created **2025-04-16** (`4c1e9ac`) as a placeholder for the gRPC
protocol's documentation. The substantive content arrived in **April 2026** with
the v2.1 documentation pass above.
