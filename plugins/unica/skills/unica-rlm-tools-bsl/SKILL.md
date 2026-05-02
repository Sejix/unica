---
name: unica-rlm-tools-bsl
description: Use when exploring large 1C BSL repositories with the rlm-tools-bsl MCP server packaged by Unica, especially when compact, token-efficient codebase analysis is more useful than diagnostics.
---

# Unica rlm-tools-bsl

Use `unica-rlm-tools-bsl` for token-efficient exploration of large 1C codebases.

## When To Use

- Use it for repository-level questions about how a large 1C configuration works.
- Use it when repeated raw file reads would flood context.
- Use `unica-bsl-workspace` instead for bsl-analyzer diagnostics, metadata checks, and source validation.

## Workflow

1. Start a session with `rlm_start(path="...", query="...")` or `rlm_start(project="...", query="...")`.
2. Use `rlm_execute(session_id="...", code="...")` for focused helper calls.
3. End the session with `rlm_end(session_id="...")` when done.
4. For large repositories, build or inspect indexes with `rlm_index` or the packaged CLI:

```sh
plugins/unica/scripts/run-rlm-bsl-index.sh index info /path/to/1c-sources
plugins/unica/scripts/run-rlm-bsl-index.sh index build /path/to/1c-sources
```

## Rules

- Do not invent project registry passwords. If `rlm_index` or `rlm_projects` asks for confirmation, ask the user for the password.
- Prefer concrete paths from the current repo. If the source root is ambiguous, inspect the repo layout before starting a session.
- Treat `rlm-tools-bsl` as a code exploration MCP, not as a replacement for bsl-analyzer diagnostics.
