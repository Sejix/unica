# Свойства объекта и RegisterRecords

Справочник native inline-операций `unica.meta.edit` для свойств существующего объекта метаданных.

## modify-property

Изменение скалярных свойств объекта. Формат: `Ключ=Значение`; несколько пар разделяются через `;;`:

```powershell
-Operation modify-property -Value "CodeLength=11 ;; DescriptionLength=150"
-Operation modify-property -Value "Hierarchical=true"
```

`modify-property` не создаёт дочерние объекты и не заменяет списковые complex properties. Для реквизитов, табличных частей и реквизитов табличных частей используй операции из `child-operations.md`.

## add-registerRecord

Добавление регистра движений документа в `RegisterRecords`. Значение — полное имя регистра в формате `MetaType.Name`; batch через `;;`:

```powershell
-Operation add-registerRecord -Value "AccumulationRegister.ОстаткиТоваров"
-Operation add-registerRecord -Value "AccumulationRegister.Продажи ;; InformationRegister.СостоянияЗаказов"
```

Операция предназначена для документов. Повторное добавление того же регистра блокируется, включая уже форматированные записи `RegisterRecords/xr:Item`.
