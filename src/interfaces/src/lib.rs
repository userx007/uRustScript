use std::error::Error;

#[derive(Debug, Default)]
pub struct Item {
    pub line: String,
    pub token_type: TokenType,
}

#[derive(Debug, Default)]
pub enum TokenType {
    #[default]
    None,
    LoadPlugin {
        name: String,
        rule: String,
        vers: String,
    },
    ConstantMacro {
        name: String,
        value: String,
    },
    VariableMacro {
        plugin: String,
        command: String,
        args: String,
        name: String,
        value: String,
    },
    Command {
        plugin: String,
        command: String,
        args: String,
    },
    IfGoTo {
        condition: String,
        label: String,
    },
    Label {
        label: String,
    },
}

pub trait Reader {
    fn read_script(
        &self,
        filepathname: &str,
        output: &mut Vec<Item>,
    ) -> Result<usize, Box<dyn Error>>;
}

pub trait Parser {
    fn parse_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>>;
}

pub trait Validator {
    fn validate_items(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>>;
}
