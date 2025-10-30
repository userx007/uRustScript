use std::error::Error;
use std::fmt;
//use regex::Regex;
use interfaces::Validator;

const RE_LOAD_PLUGIN: &'static str =
    r#"(^LOAD_PLUGIN\s+[A-Za-z_]+(_[A-Za-z_]+)?(\s+(<=|<|>=|>|==)\s+v\d+\.\d+\.\d+\.\d+)?$)"#;

const RE_CONST_MACRO: &'static str = r#"(^[A-Za-z_][A-Za-z0-9_]*\s*:=\s*\S.*$)"#;

const RE_VAR_MACRO: &'static str =
    r#"(^[A-Za-z_][A-Za-z0-9_]*\s*\?=\s*[A-Z]+[A-Z0-9_]*[A-Z]+\.[A-Z]+[A-Z0-9_]*[A-Z]+.*$)"#;

const RE_COMMAND: &'static str = r#"(^[A-Z]+[A-Z0-9_]*[A-Z]+\.[A-Z]+[A-Z0-9_]*[A-Z]+\s*.*$)"#;

const RE_IF_GOTO_OR_GOTO: &'static str =
    r#"(^(?:IF\s+\S(?:.*\S)?\s+)?GOTO\s+[A-Za-z_][A-Za-z0-9_]*$)"#;

const RE_LABEL: &'static str = r#"LABEL\s+[A-Za-z_][A-Za-z0-9_]*$)"#;

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

    fn is_load_plugin(&self, line: &str) -> bool {
        //        let re = Regex::new(RE_LOAD_PLUGIN).unwrap();
        //        re.is_match(line)
        true
    }

    fn is_const_macro(&self, line: &str) -> bool {
        //        let re = Regex::new(RE_CONST_MACRO).unwrap();
        //        re.is_match(line)
        true
    }

    fn is_var_macro(&self, line: &str) -> bool {
        //        let re = Regex::new(RE_VAR_MACRO).unwrap();
        //        re.is_match(line)
        true
    }

    fn is_command(&self, line: &str) -> bool {
        //        let re = Regex::new(RE_COMMAND).unwrap();
        //        re.is_match(line)
        true
    }

    fn is_ifgoto_or_goto(&self, line: &str) -> bool {
        //        let re = Regex::new(RE_IF_GOTO_OR_GOTO).unwrap();
        //        re.is_match(line)
        true
    }

    fn is_label(&self, line: &str) -> bool {
        //        let re = Regex::new(RE_LABEL).unwrap();
        //        re.is_match(line)
        true
    }

    fn validate_line(&self, line: &str) -> bool {
        if !self.is_load_plugin(line)
            && !self.is_const_macro(line)
            && !self.is_var_macro(line)
            && !self.is_command(line)
            && !self.is_ifgoto_or_goto(line)
            && !self.is_label(line)
        {
            println!("Invalid statement [{}]", line);
            return false;
        }
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, input: &Vec<String>) -> Result<(), Box<dyn Error>> {
        for line in input {
            if !self.validate_line(line) {
                return Err(Box::new(ValidateError::InvalidStatement));
            }
        }
        Ok(())
    }
}
