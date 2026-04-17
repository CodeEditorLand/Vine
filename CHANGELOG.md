# Changelog

All notable changes to the Vine element are documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/).

Vine is the gRPC/protobuf protocol definition — the contract between Mountain
(Rust backend) and Cocoon (TypeScript extension host), Wind (UI services),
and Air (background daemon). The canonical `Vine.proto` lives in
`Mountain/Proto/` and is consumed by all ecosystem components.

## [v2.1] — Q2 2026: Documentation Polish

### Changed

- GitHub URLs corrected in README documentation
- Table formatting and alignment improved in architecture docs
- Park emoji added to ecosystem section header for visual consistency

## [v2.0] — Q1 2026: Protocol Stabilization

### Added

- MountainService: 3 generic RPCs (ProcessCocoonRequest,
  SendCocoonNotification, CancelOperation)
- CocoonService: 71 RPCs covering full VS Code extension host surface
  - Language features: 42 RPCs (hover, completion, definition, references,
    code actions, document/workspace symbols, rename, formatting, signature
    help, code lens, folding, selection, semantic tokens, inlay hints, type
    hierarchy, call hierarchy, linked editing)
  - Window/UI: 15 RPCs (quick pick, input box, progress, webview, status bar,
    message dialogs, terminal opening)
  - File system: 10 RPCs (CRUD, watch, find, stat, edit apply)
  - Terminal: 4 RPCs (open, input, close, process events)
  - TreeView, SCM, Debug, Secrets: full coverage
- Field naming: PascalCase with `keepCase: true` proto-loader option

## [v1.2] — Q3-Q4 2025: Contract Evolution

### Changed

- RPC signatures aligned with handler implementations in Mountain and Cocoon
- DTO deserialization patterns standardized across all service messages

## [v1.1] — Q2 2025: Project Inception

### Added

- Repository created April 2025 as documentation/specification companion to
  `Mountain/Proto/Vine.proto`
- Architecture diagrams and ecosystem overview
- License (CC0) and code of conduct
