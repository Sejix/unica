# Inline-операции над дочерними элементами

Подробный справочник native inline-операций `unica.meta.edit` для дочерних элементов объекта метаданных.

## Общие правила

**Batch-режим** — несколько элементов через `;;`:
```
-Value "Комментарий: Строка(200) ;; Сумма: Число(15,2) | index"
```

**Shorthand-формат** реквизитов: `ИмяРеквизита: Тип | флаги`

Флаги: `req` (FillChecking=ShowError), `index` (Indexing=Index).

## Составные типы

Для реквизитов с несколькими допустимыми типами — разделитель `+`:
```powershell
-Operation add-attribute -Value "Значение: Строка + Число(15,2) + Дата + CatalogRef.Контрагенты"
-Operation add-attribute -Value "Значение: Строка + Число(15,2) | req"
-Operation modify-ts-attribute -Value "Данные.Значение: type=Строка + Число(15,2) + Дата"
```

## add-attribute

```powershell
-Operation add-attribute -Value "Комментарий: Строка(200)"
-Operation add-attribute -Value "Сумма: Число(15,2) | req, index"
-Operation add-attribute -Value "Ном: CatalogRef.Номенклатура | req ;; Кол: Число(15,3)"
```

## add-ts

Формат: `ИмяТЧ: Реквизит1: Тип1, Реквизит2: Тип2, ...`

```powershell
-Operation add-ts -Value "Товары: Ном: CatalogRef.Ном | req, Кол: Число(15,3), Цена: Число(15,2), Сумма: Число(15,2)"
```

## add-ts-attribute / remove-ts-attribute / modify-ts-attribute

Операции над реквизитами **внутри существующей ТЧ**. Формат: `ИмяТЧ.ОпределениеРеквизита` (dot-нотация).

```powershell
# Добавить реквизит в ТЧ
-Operation add-ts-attribute -Value "Товары.СтавкаНДС: EnumRef.СтавкиНДС"
-Operation add-ts-attribute -Value "Товары.Скидка: Число(15,2) ;; Товары.Бонус: Число(15,2)"

# Удалить реквизит из ТЧ
-Operation remove-ts-attribute -Value "Товары.УстаревшийРекв"
-Operation remove-ts-attribute -Value "Товары.Рекв1 ;; Товары.Рекв2"

# Изменить реквизит в ТЧ (rename, type change и т.д.)
-Operation modify-ts-attribute -Value "Товары.СтароеИмя: name=НовоеИмя, type=Строка(500)"
```

Batch через `;;` — можно указать разные ТЧ: `"Товары.А: Строка(50) ;; Услуги.Б: Число(10)"`.

## modify-ts

Изменение свойств **самой табличной части** (`name`, `synonym`, `comment`, `fillChecking`, `use`):

```powershell
-Operation modify-ts -Value "Товары: synonym=Товарный состав"
-Operation modify-ts -Value "Товары: fillChecking=ShowError"
```

Формат: `ИмяТЧ: ключ=значение, ключ=значение`. Ключи `type`, `indexing`, `allowedSign` применимы к реквизитам, но не к самой табличной части.

## modify-attribute

Формат: `ИмяЭлемента: ключ=значение, ключ=значение`

Ключи: `name` (rename), `type`, `synonym`, `comment`, `indexing`, `fillChecking`, `use`, `allowedSign`.

```powershell
-Operation modify-attribute -Value "СтароеИмя: name=НовоеИмя, type=Строка(500)"
-Operation modify-attribute -Value "Комментарий: indexing=Index"
```
