---
name: test-authoring
description: "Проектирование и запуск тестов 1С: test-authoring, yaxunit-test и Vanessa Automation. Используй когда нужно написать тест, подобрать сценарии, запустить all/module тесты или проверить изменение через Unica runtime."
---

# Test Authoring

## MCP routing

- Preferred path: use MCP `unica` tools `unica.code.search`, `unica.project.map`, `unica.runtime.execute`, and the relevant `unica.*.info` tools.
- Use `unica.standards.search` or `unica.standards.explain` when test design depends on a platform or standards rule.
- Do not call internal runtime, analyzer, or package adapters directly. They are hidden behind MCP `unica`.

## Workflow

1. Define the behavior under test before choosing the framework: pure BSL unit, object lifecycle, form behavior, integration contract, or regression around a diagnostic.
2. Search existing tests and fixtures with `unica.code.search`; follow local naming, setup, teardown, and assertion style.
3. Prefer YaXUnit for module/unit-level BSL behavior and Vanessa Automation for UI/business scenarios that require a client.
4. Build the smallest stable fixture. Avoid dependence on production data unless the user explicitly requests an integration test.
5. Run `unica.runtime.execute` with `operation=syntax` after adding test code, then `operation=test` with `testRunner=yaxunit` or `testRunner=va`.
6. Report exact failing test, expected/actual behavior, and whether the failure is test setup or product behavior.

## MCP examples

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.runtime.execute",
    "arguments": {
      "cwd": "<workspace>",
      "operation": "test",
      "testRunner": "yaxunit",
      "testScope": "module",
      "module": "ТестДокументаЗаказКлиента",
      "dryRun": false
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
      "operation": "test",
      "testRunner": "va",
      "testScope": "all",
      "dryRun": false
    }
  }
}
```
