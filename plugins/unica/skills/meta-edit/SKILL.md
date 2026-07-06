---
name: meta-edit
description: Точечное редактирование объекта метаданных 1С. Используй когда нужно добавить или изменить реквизиты, табличные части, реквизиты табличных частей, регистры движений документа или свойства существующего объекта конфигурации
argument-hint: <ObjectPath> -Operation <op> -Value "<val>"
allowed-tools:
  - Bash
  - Read
  - Write
  - Glob
---

# /meta-edit — точечное редактирование метаданных 1С

## MCP routing

- Preferred path: use MCP `unica` tool `unica.meta.edit`; `unica` owns XML mutations and refreshes related workspace caches after mutations.
- Do not call internal MCP/CLI adapters directly. They are hidden behind `unica` and synchronized by the orchestrator.
- Execution path: call MCP `unica` tool `unica.meta.edit`; skill-local operation scripts are not part of the workflow.
- For mutating operations, pass `dryRun: false` only when the user explicitly requested the change; otherwise keep the default dry run.
- Vendor support guard runs inside `unica`; if it blocks a locked/read-only supported object, prefer CFE/release-support or an explicit support-state change plan instead of editing raw support metadata.

Атомарные операции модификации существующих XML объектов метаданных.

## MCP вызов

### Inline mode: простая операция

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "<path>",
      "Operation": "modify-property",
      "Value": "DescriptionLength=150",
      "dryRun": false
    }
  }
}
```

## Операции — сводная таблица

Native `unica.meta.edit` поддерживает только inline `Operation` + `Value`; `DefinitionFile`/JSON DSL не входит в текущую MCP-поверхность. Batch через `;;` поддержан для операций ниже, кроме `modify-property`, где `;;` разделяет пары свойств.

### Дочерние элементы — [child-operations.md](child-operations.md)

| Операция | Формат Value | Пример |
|----------|-------------|--------|
| `add-attribute` | `Имя: Тип \| флаги` | `"Сумма: Число(15,2) \| req, index"` |
| `add-ts` | `ТЧ: Рекв1: Тип1, Рекв2: Тип2` | `"Товары: Ном: CatalogRef.Ном, Кол: Число(15,3)"` |
| `add-ts-attribute` | `ТЧ.Имя: Тип` | `"Товары.Скидка: Число(15,2)"` |
| `remove-ts-attribute` | `ТЧ.Имя` | `"Товары.УстаревшийРекв"` |
| `modify-attribute` | `Имя: ключ=значение` | `"СтароеИмя: name=НовоеИмя, type=Строка(500)"` |
| `modify-ts-attribute` | `ТЧ.Имя: ключ=значение` | `"Товары.Рекв: name=НовоеИмя"` |
| `modify-ts` | `ТЧ: ключ=значение` | `"Товары: synonym=Товарный состав"` |

### Свойства объекта — [properties-reference.md](properties-reference.md)

| Операция | Формат Value | Пример |
|----------|-------------|--------|
| `modify-property` | `Ключ=Значение` | `"CodeLength=11 ;; DescriptionLength=150"` |
| `add-registerRecord` | `MetaType.Name` | `"AccumulationRegister.ОстаткиТоваров"` |

## Быстрые примеры

### Добавить реквизиты

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Catalogs/Контрагенты/Контрагенты.xml",
      "Operation": "add-attribute",
      "Value": "Комментарий: Строка(200) ;; Сумма: Число(15,2) | index",
      "dryRun": false
    }
  }
}
```

### Составной тип

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Catalogs/Контрагенты/Контрагенты.xml",
      "Operation": "add-attribute",
      "Value": "Значение: Строка + Число(15,2) + Дата + CatalogRef.Контрагенты",
      "dryRun": false
    }
  }
}
```

### Добавить табличную часть с реквизитами

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Documents/ЗаказПокупателя/ЗаказПокупателя.xml",
      "Operation": "add-ts",
      "Value": "Товары: Ном: CatalogRef.Ном | req, Кол: Число(15,3), Цена: Число(15,2)",
      "dryRun": false
    }
  }
}
```

### Удалить реквизит табличной части

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Documents/ЗаказПокупателя/ЗаказПокупателя.xml",
      "Operation": "remove-ts-attribute",
      "Value": "Товары.УстаревшийРеквизит",
      "dryRun": false
    }
  }
}
```

### Переименовать и сменить тип

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Catalogs/Контрагенты/Контрагенты.xml",
      "Operation": "modify-attribute",
      "Value": "СтароеИмя: name=НовоеИмя, type=Строка(500)",
      "dryRun": false
    }
  }
}
```

### Изменить свойства объекта

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Catalogs/Контрагенты/Контрагенты.xml",
      "Operation": "modify-property",
      "Value": "CodeLength=11 ;; DescriptionLength=150",
      "dryRun": false
    }
  }
}
```

### Добавить регистр движений документа

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Documents/ЗаказПокупателя/ЗаказПокупателя.xml",
      "Operation": "add-registerRecord",
      "Value": "AccumulationRegister.ОстаткиТоваров",
      "dryRun": false
    }
  }
}
```

### Изменить свойства табличной части

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.edit",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "src/Documents/ЗаказПокупателя/ЗаказПокупателя.xml",
      "Operation": "modify-ts",
      "Value": "Товары: synonym=Товарный состав, fillChecking=ShowError",
      "dryRun": false
    }
  }
}
```

## Верификация

### Валидация после редактирования

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.validate",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "<ObjectPath>"
    }
  }
}
```

### Сводка объекта

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "unica.meta.info",
    "arguments": {
      "cwd": "<workspace>",
      "ObjectPath": "<ObjectPath>"
    }
  }
}
```
