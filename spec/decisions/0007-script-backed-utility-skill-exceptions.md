# 0007. Script-backed utility skill exceptions

## Status

Accepted.

## Context

Unica skills are MCP-first: prompt-visible skills should route through the single public `unica` MCP server and native `unica.*` tools once a capability has crossed that boundary.

Two current utility skills are intentionally different:

- `web-test`
- `img-grid`

They are not XML/DSL configuration operations. `web-test` owns browser automation assets and Playwright workflow glue; `img-grid` owns image-analysis helper behavior. Moving either behind `unica.*` would be a separate product change, not a cleanup required for native XML parity.

## Decision

`web-test` and `img-grid` are a permanent local-tool exception until a dedicated ADR replaces this one.

They may keep script-backed implementation details if all of the following remain true:

- they do not reintroduce direct script execution guidance for native XML/DSL operations;
- packaging guardrails prevent local dependencies, browser sessions, screenshots, videos, and generated artifacts from entering release archives by accident;
- provenance marks them as explicit exceptions rather than migrated `unica.*` tools;
- CI keeps checking that migrated skills stay MCP-first.

## Consequences

This keeps native XML/DSL architecture clean without pretending every utility must become a Rust MCP operation immediately.

Future migration to `unica.web_test.*` or `unica.img_grid.*` should be planned as new functionality with explicit public contracts, not mixed into packaging or safety refactors.
