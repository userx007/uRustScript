use interfaces::{Validator, Item, TokenType};
use regex::Regex;
use std::error::Error;
use std::fmt;

const RE_LOAD_PLUGIN: &'static str =
    r#"(^LOAD_PLUGIN\s+[A-Za-z_]+(_[A-Za-z_]+)?(\s+(<=|<|>=|>|==)\s+v\d+\.\d+\.\d+\.\d+)?$)"#;

const RE_CONST_MACRO: &'static str = r#"(^[A-Za-z_][A-Za-z0-9_]*\s*:=\s*\S.*$)"#;

const RE_VAR_MACRO: &'static str =
    r#"(^[A-Za-z_][A-Za-z0-9_]*\s*\?=\s*[A-Z]+[A-Z0-9_]*[A-Z]+\.[A-Z]+[A-Z0-9_]*[A-Z]+.*$)"#;

const RE_COMMAND: &'static str = r#"(^[A-Z]+[A-Z0-9_]*[A-Z]+\.[A-Z]+[A-Z0-9_]*[A-Z]+\s*.*$)"#;

const RE_IF_GOTO_OR_GOTO: &'static str =
    r#"(^(?:IF\s+\S(?:.*\S)?\s+)?GOTO\s+[A-Za-z_][A-Za-z0-9_]*$)"#;

const RE_LABEL: &'static str = r#"(LABEL\s+[A-Za-z_][A-Za-z0-9_]*$)"#;

#[derive(Debug)]
enum ValidateError {
    InvalidStatement,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateError::InvalidStatement => write!(f, "Invalid statement in script"),
        }
    }
}

impl Error for ValidateError {}

pub struct ScriptValidator;

impl ScriptValidator {
    pub fn new() -> Self {
        ScriptValidator {}
    }

    fn is_load_plugin(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_LOAD_PLUGIN).unwrap();
        if re.is_match(&item.line) {
            item.token_type = TokenType::LoadPlugin { name: "".into() };
            return true;
        }
        false
    }

    fn is_const_macro(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_CONST_MACRO).unwrap();
        if re.is_match(&item.line) {
            item.token_type = TokenType::ConstantMacro {
                name: "".into(),
                value: "".into(),
            };
            return true;
        }
        false
    }

    fn is_var_macro(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_VAR_MACRO).unwrap();
        if re.is_match(&item.line) {
            item.token_type = TokenType::VariableMacro {
                plugin: "".into(),
                command: "".into(),
                args: "".into(),
                name: "".into(),
                value: "".into(),
            };
            return true;
        }
        false
    }

    fn is_command(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_COMMAND).unwrap();
        if re.is_match(&item.line) {
            item.token_type = TokenType::Command {
                plugin: "".into(),
                command: "".into(),
                args: "".into(),
            };
            return true;
        }
        false
    }

    fn is_if_cond_goto(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_IF_GOTO_OR_GOTO).unwrap();
        if re.is_match(&item.line) {
            item.token_type = TokenType::IfGoTo {
                condition: "".into(),
                label: "".into(),
            };
            return true;
        }
        false
    }

    fn is_label(&self, item: &mut Item) -> bool {
        let re = Regex::new(RE_LABEL).unwrap();
        if re.is_match(&item.line) {
            item.token_type = TokenType::Label { label: "".into() };
            return true;
        }
        false
    }

    fn validate_item(&self, item: &mut Item) -> bool {
        println!("\tValidating item {}", item.line);
        if !self.is_load_plugin(item)
            && !self.is_const_macro(item)
            && !self.is_var_macro(item)
            && !self.is_command(item)
            && !self.is_if_cond_goto(item)
            && !self.is_label(item)
        {
            println!("Invalid statement [{:?}]", item);
            return false;
        }
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, inout: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Validating script ...");
        for item in inout {
            if !self.validate_item(item) {
                return Err(Box::new(ValidateError::InvalidStatement));
            }
        }
        Ok(())
    }
}
