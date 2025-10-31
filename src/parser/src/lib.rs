use interfaces::{Item, Parser, TokenType};
use regex::Regex;
use std::error::Error;
use std::fmt;

const RE_LOAD_PLUGIN: &'static str =
    r#"^LOAD_PLUGIN\s+([A-Z0-9_]+)(?:\s*(<=|<|>=|>|==)\s*(v\d+\.\d+\.\d+\.\d+))?$"#;
const RE_CONST_MACRO: &'static str = r#"^([A-Za-z_][A-Za-z0-9_]*)\s*:=\s*(.+)$"#;
const RE_VAR_MACRO: &'static str =
    r#"^([A-Za-z_][A-Za-z0-9_]*)\s*\?=\s*([A-Z0-9_]+)\.([A-Z]+[A-Z0-9_]*)(?:\s+(.*))?$"#;
const RE_COMMAND: &'static str = r#"^([A-Z0-9_]+)\.([A-Z]+[A-Z0-9_]*)(?:\s+(.*))?$"#;
const RE_IF_GOTO_OR_GOTO: &'static str = r#"^(?:IF\s+(.*?)\s+)?GOTO\s+([A-Za-z0-9_]*)\s*$"#;
const RE_LABEL: &'static str = r#"^(LABEL\s+[A-Za-z0-9_]*$)"#;

#[derive(Debug)]
enum ParseError {
    InvalidStatement,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidStatement => write!(f, "Invalid item in script"),
        }
    }
}

impl Error for ParseError {}

pub struct ScriptParser;

impl ScriptParser {
    pub fn new() -> Self {
        ScriptParser {}
    }

    fn is_load_plugin(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_LOAD_PLUGIN).unwrap();
        if let Some(caps) = re.captures(&item.line) {
            let name = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let rule = caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let vers = caps
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            item.token_type = TokenType::LoadPlugin { name, rule, vers };
            return true;
        }
        false
    }

    fn is_const_macro(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_CONST_MACRO).unwrap();
        if let Some(caps) = re.captures(&item.line) {
            let name = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let value = caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            item.token_type = TokenType::ConstantMacro { name, value };
            return true;
        }
        false
    }

    fn is_var_macro(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_VAR_MACRO).unwrap();
        if let Some(caps) = re.captures(&item.line) {
            let name = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let plugin = caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let command = caps
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let args = caps
                .get(4)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let value = "".into();

            item.token_type = TokenType::VariableMacro {
                plugin,
                command,
                args,
                name,
                value,
            };
            return true;
        }
        false
    }

    fn is_command(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_COMMAND).unwrap();
        if let Some(caps) = re.captures(&item.line) {
            let plugin = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let command = caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let args = caps
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            item.token_type = TokenType::Command {
                plugin,
                command,
                args,
            };
            return true;
        }
        false
    }

    fn is_if_cond_goto(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_IF_GOTO_OR_GOTO).unwrap();
        if let Some(caps) = re.captures(&item.line) {
            let condition = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            let label = caps
                .get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            item.token_type = TokenType::IfGoTo { condition, label };
            return true;
        }
        false
    }

    fn is_label(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_LABEL).unwrap();
        if let Some(caps) = re.captures(&item.line) {
            let label = caps
                .get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            item.token_type = TokenType::Label { label };
            return true;
        }
        false
    }

    fn parse_item(&self, item: &mut Item) -> bool {
        if !self.is_load_plugin(item)
            && !self.is_const_macro(item)
            && !self.is_var_macro(item)
            && !self.is_command(item)
            && !self.is_if_cond_goto(item)
            && !self.is_label(item)
        {
            println!("Invalid item [{:?}]", item);
            return false;
        }
        // free the memory used by line
        item.line = String::new();
        true
    }
}

impl Parser for ScriptParser {
    fn parse_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Parsing script ...");
        for item in items {
            if !self.parse_item(item) {
                return Err(Box::new(ParseError::InvalidStatement));
            }
        }
        Ok(())
    }
}
