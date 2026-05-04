---
name: code-diagnostics
description: "Диагностика BSL и объяснение отключений диагностик в коде. Используй когда нужно запустить или разобрать диагностики, объяснить коды АПК, EDT, BSL LS, inline/range disable markers, suppression-комментарии или стандарт v8std за диагностикой."
---

# Code Diagnostics

## MCP routing

- Preferred path: use MCP `unica` tools `unica.code.diagnostics`, `unica.code.search`, `unica.standards.explain`, `unica.standards.search`, and `unica.runtime.execute`.
- v8std access goes only through public `unica.standards.*` tools.
- Do not call internal analyzer, standards, or package adapters directly. They are hidden behind MCP `unica`.

## Workflow

1. Run `unica.code.diagnostics` for the selected source-set or module. If syntax validity is relevant, also run `unica.runtime.execute` with `operation=syntax`.
2. Group diagnostics by file, diagnostic id/code, and root cause. Do not fix duplicate reports independently when one source issue explains them.
3. Search nearby code with `unica.code.search` when the diagnostic output lacks enough context.
4. For each diagnostic id/code, call `unica.standards.explain` with `codes` when the code is explicit; otherwise search `unica.standards.search` by diagnostic name, APK/EDT/BSL LS token, or nearby snippet.
5. Report fixes in cause-first order: source defect, impacted diagnostics, standard reference, verification command.

## Suppression and range-disable comments

When comments disable diagnostics over a line or range, treat the exact marker as evidence, not as decoration.

- Extract literal codes or ids from the comment: АПК, EDT, BSL LS, analyzer rule names, numeric or mnemonic ids.
- Use `unica.standards.explain` with all extracted codes. If v8std does not resolve a code, search with `unica.standards.search` using the code plus nearby diagnostic text.
- Explain why the отключение exists only when the code, surrounding range, and standard support the reason. If the reason is absent, say that the suppression is not justified in the source.
- Prefer narrowing the disabled range or fixing the code. Keep suppression only when the standard or platform limitation makes the diagnostic intentionally false-positive.

## MCP examples

```jsonc
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.code.diagnostics",
    "arguments": {
      "cwd": "<workspace>",
      "sourceDir": "src",
      "limit": 100
    }
  }
}
```

```jsonc
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.standards.explain",
    "arguments": {
      "codes": ["АПК:142", "LineLength"]
    }
  }
}
```

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.runtime.execute",
    "arguments": {
      "cwd": "<workspace>",
      "operation": "syntax",
      "mode": "designer-modules",
      "dryRun": false
    }
  }
}
```
