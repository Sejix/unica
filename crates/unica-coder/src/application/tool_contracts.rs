use super::{ToolHandler, ToolSpec};
use crate::domain::workspace::WorkspaceContext;
use crate::infrastructure::path_policy::WorkspacePathPolicy;
use serde_json::{json, Map, Value};
use std::collections::BTreeSet;

const COMMON_ARGS: &[&str] = &["cwd", "dryRun", "confirm"];

const NATIVE_XML_DSL_ARGS: &[&str] = &[
    "BaseForm",
    "Batch",
    "BodyLimit",
    "BorrowMainAttribute",
    "Child",
    "Children",
    "CIPath",
    "Columns",
    "Command",
    "CommandName",
    "CompatibilityMode",
    "ConfigDir",
    "ConfigPath",
    "Context",
    "CreateIfMissing",
    "DataPath",
    "DefinitionFile",
    "Detailed",
    "ExtensionPath",
    "Expand",
    "Field",
    "Fields",
    "Force",
    "FormName",
    "FormPath",
    "Format",
    "FromObject",
    "InterceptorType",
    "JsonPath",
    "KeepFiles",
    "Kind",
    "Language",
    "Limit",
    "IsFunction",
    "MaxErrors",
    "MaxParams",
    "MethodName",
    "MetadataPath",
    "Mode",
    "ModulePath",
    "Name",
    "NamePrefix",
    "NoRole",
    "NoValidate",
    "Object",
    "ObjectName",
    "ObjectPath",
    "Offset",
    "Operation",
    "OutFile",
    "OutputDir",
    "OutputPath",
    "Parent",
    "Path",
    "ProcessorName",
    "Purpose",
    "RightsPath",
    "Section",
    "SetDefault",
    "SetMainSKD",
    "ShowDenied",
    "SrcDir",
    "SubsystemPath",
    "Synonym",
    "TemplateName",
    "TemplatePath",
    "TemplateType",
    "Type",
    "Value",
    "Vendor",
    "Version",
    "WithText",
    "baseForm",
    "batch",
    "bodyLimit",
    "borrowMainAttribute",
    "child",
    "children",
    "ciPath",
    "columns",
    "command",
    "commandName",
    "compatibilityMode",
    "configDir",
    "configPath",
    "context",
    "createIfMissing",
    "dataPath",
    "definitionFile",
    "detailed",
    "extensionPath",
    "expand",
    "field",
    "fields",
    "force",
    "formName",
    "formPath",
    "format",
    "fromObject",
    "interceptorType",
    "jsonPath",
    "keepFiles",
    "kind",
    "language",
    "limit",
    "isFunction",
    "maxErrors",
    "maxParams",
    "methodName",
    "metadataPath",
    "mode",
    "modulePath",
    "name",
    "namePrefix",
    "noRole",
    "noValidate",
    "object",
    "objectName",
    "objectPath",
    "offset",
    "operation",
    "outFile",
    "outputDir",
    "outputPath",
    "parent",
    "path",
    "processorName",
    "purpose",
    "rightsPath",
    "section",
    "setDefault",
    "setMainSKD",
    "showDenied",
    "srcDir",
    "subsystemPath",
    "synonym",
    "templateName",
    "templatePath",
    "templateType",
    "type",
    "value",
    "vendor",
    "version",
    "withText",
];

const BUILD_ARGS: &[&str] = &[
    "config",
    "database",
    "dbPassword",
    "dbUser",
    "format",
    "infobase",
    "mode",
    "password",
    "path",
    "sourceDir",
    "sourceSet",
    "target",
    "user",
];

const CODE_ARGS: &[&str] = &[
    "config",
    "format",
    "limit",
    "mode",
    "path",
    "query",
    "sourceDir",
];

const STANDARDS_ARGS: &[&str] = &[
    "body_limit",
    "bodyLimit",
    "codes",
    "id",
    "idOrAliasOrUrl",
    "language",
    "limit",
    "mode",
    "query",
    "snippet",
    "types",
];

pub fn input_schema_for_tool(tool: &ToolSpec) -> Value {
    let property_names = allowed_args(tool);
    let mut properties = Map::new();
    for name in property_names {
        properties.insert(name.to_string(), property_schema(name));
    }

    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": properties,
        "required": required_args(tool),
    })
}

pub fn validate_tool_arguments(
    tool: ToolSpec,
    args: &Map<String, Value>,
    dry_run: bool,
) -> Result<(), String> {
    let allowed = allowed_args(&tool).into_iter().collect::<BTreeSet<_>>();
    for key in args.keys() {
        if !allowed.contains(key.as_str()) {
            return Err(format!(
                "{} does not accept argument `{key}`; use typed MCP arguments only",
                tool.name
            ));
        }
    }
    for (key, value) in args {
        validate_argument_type(tool.name, key, value)?;
    }

    if !dry_run {
        for required in required_args(&tool) {
            if !args.contains_key(required) {
                return Err(format!("{} requires `{required}` argument", tool.name));
            }
        }
    }

    Ok(())
}

pub fn validate_workspace_paths(
    tool: ToolSpec,
    args: &Map<String, Value>,
    dry_run: bool,
    context: &WorkspaceContext,
) -> Result<(), String> {
    if dry_run || !tool.mutating {
        return Ok(());
    }
    if !matches!(tool.handler, ToolHandler::NativeOperation { .. }) {
        return Ok(());
    }

    let policy = WorkspacePathPolicy::new(context);
    for key in native_write_path_args(tool) {
        if let Some(Value::String(path)) = args.get(*key) {
            policy.resolve_write(path.as_str())?;
        }
    }
    Ok(())
}

fn native_write_path_args(tool: ToolSpec) -> &'static [&'static str] {
    match tool.handler {
        ToolHandler::NativeOperation { operation, .. } => match operation {
            "cf-init" | "cfe-init" => &["OutputDir", "outputDir"],
            "cf-edit" => &["ConfigPath", "configPath", "Path", "path"],
            "meta-compile" => &["OutputDir", "outputDir"],
            "meta-edit" => &["ObjectPath", "objectPath", "Path", "path"],
            "meta-remove" => &["ConfigDir", "configDir"],
            "form-add" => &["ObjectPath", "objectPath"],
            "form-compile" => &["OutputPath", "outputPath"],
            "form-edit" => &["FormPath", "formPath"],
            "form-remove" => &["SrcDir", "srcDir"],
            "interface-edit" => &["CIPath", "ciPath"],
            "subsystem-compile" => &["OutputDir", "outputDir", "Parent", "parent"],
            "subsystem-edit" => &["SubsystemPath", "subsystemPath"],
            "template-add" | "template-remove" => &["SrcDir", "srcDir"],
            "skd-compile" | "mxl-compile" => &["OutputPath", "outputPath"],
            "skd-edit" => &["TemplatePath", "templatePath"],
            "role-compile" => &["OutputDir", "outputDir"],
            "cfe-borrow" | "cfe-patch-method" => &["ExtensionPath", "extensionPath"],
            _ => &[],
        },
        _ => &[],
    }
}

fn allowed_args(tool: &ToolSpec) -> Vec<&'static str> {
    let mut names = COMMON_ARGS.to_vec();
    match tool.handler {
        ToolHandler::NativeOperation { operation, .. } => {
            names.extend(native_args_for(operation));
        }
        ToolHandler::BuildRuntime { .. } => names.extend(BUILD_ARGS),
        ToolHandler::CodeAdapter { .. } => names.extend(CODE_ARGS),
        ToolHandler::StandardsAdapter { .. } => names.extend(STANDARDS_ARGS),
        ToolHandler::ProjectStatus => {}
        ToolHandler::LegacyScript { .. } => names.extend(NATIVE_XML_DSL_ARGS),
    }
    names.sort_unstable();
    names.dedup();
    names
}

fn native_args_for(_operation: &str) -> &'static [&'static str] {
    NATIVE_XML_DSL_ARGS
}

fn required_args(tool: &ToolSpec) -> Vec<&'static str> {
    match tool.handler {
        ToolHandler::NativeOperation { operation, .. } => match operation {
            "cf-info" | "cf-validate" => vec!["ConfigPath"],
            "cfe-diff" => vec!["ExtensionPath", "ConfigPath"],
            "cfe-validate" => vec!["ExtensionPath"],
            "meta-info" | "meta-validate" | "meta-edit" => vec!["ObjectPath"],
            "form-info" | "form-validate" | "form-edit" => vec!["FormPath"],
            "interface-validate" | "interface-edit" => vec!["CIPath"],
            "subsystem-info" | "subsystem-validate" | "subsystem-edit" => vec!["SubsystemPath"],
            "skd-info" | "skd-validate" | "skd-edit" => vec!["TemplatePath"],
            "mxl-info" | "mxl-validate" | "mxl-decompile" => vec!["TemplatePath"],
            "role-info" | "role-validate" => vec!["RightsPath"],
            _ => Vec::new(),
        },
        ToolHandler::StandardsAdapter {
            operation: "search",
            ..
        } => vec!["query"],
        _ => Vec::new(),
    }
}

fn property_schema(name: &str) -> Value {
    let value_type = if matches!(
        name,
        "dryRun"
            | "confirm"
            | "Detailed"
            | "detailed"
            | "Force"
            | "force"
            | "NoValidate"
            | "noValidate"
            | "NoRole"
            | "noRole"
            | "WithText"
            | "withText"
            | "CreateIfMissing"
            | "createIfMissing"
            | "IsFunction"
            | "isFunction"
    ) {
        "boolean"
    } else if matches!(
        name,
        "limit" | "Offset" | "offset" | "MaxParams" | "maxParams"
    ) {
        "integer"
    } else if matches!(
        name,
        "codes" | "types" | "Fields" | "fields" | "Children" | "children"
    ) {
        "array"
    } else {
        "string"
    };

    if value_type == "array" {
        json!({ "type": "array", "items": { "type": "string" } })
    } else {
        json!({ "type": value_type })
    }
}

fn validate_argument_type(tool_name: &str, key: &str, value: &Value) -> Result<(), String> {
    let expected = expected_scalar_type(key);
    match expected {
        Some("boolean") if !value.is_boolean() => {
            Err(format!("{tool_name} argument `{key}` must be boolean"))
        }
        Some("integer") if value.as_i64().is_none() => {
            Err(format!("{tool_name} argument `{key}` must be integer"))
        }
        _ => Ok(()),
    }
}

fn expected_scalar_type(key: &str) -> Option<&'static str> {
    if matches!(
        key,
        "dryRun"
            | "confirm"
            | "Detailed"
            | "detailed"
            | "Force"
            | "force"
            | "NoValidate"
            | "noValidate"
            | "NoRole"
            | "noRole"
            | "WithText"
            | "withText"
            | "CreateIfMissing"
            | "createIfMissing"
            | "IsFunction"
            | "isFunction"
    ) {
        Some("boolean")
    } else if matches!(
        key,
        "limit" | "Offset" | "offset" | "MaxParams" | "maxParams"
    ) {
        Some("integer")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::tools;

    #[test]
    fn native_contracts_reject_unknown_args() {
        let tool = tools()
            .into_iter()
            .find(|tool| tool.name == "unica.cf.info")
            .unwrap();
        let mut args = Map::new();
        args.insert("ConfigPath".to_string(), json!("Configuration.xml"));
        args.insert("unknown".to_string(), json!("value"));

        let error = validate_tool_arguments(tool, &args, false).unwrap_err();

        assert!(error.contains("does not accept argument `unknown`"));
    }

    #[test]
    fn mutating_dry_run_does_not_require_payload() {
        let tool = tools()
            .into_iter()
            .find(|tool| tool.name == "unica.form.edit")
            .unwrap();
        let args = Map::new();

        validate_tool_arguments(tool, &args, true).unwrap();
    }

    #[test]
    fn contracts_reject_wrong_scalar_type() {
        let tool = tools()
            .into_iter()
            .find(|tool| tool.name == "unica.cf.info")
            .unwrap();
        let mut args = Map::new();
        args.insert("ConfigPath".to_string(), json!("Configuration.xml"));
        args.insert("dryRun".to_string(), json!("false"));

        let error = validate_tool_arguments(tool, &args, false).unwrap_err();

        assert!(error.contains("dryRun"));
        assert!(error.contains("boolean"));
    }
}
