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
        plugin: String,
        rule: String,
        vers: String,
    },
    ConstantMacro {
        cmacro: String,
        value: String,
    },
    VariableMacro {
        plugin: String,
        command: String,
        args: String,
        vmacro: String,
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
    fn parse_script(&mut self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>>;
}

pub trait Validator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>>;
}


pub trait Runner {
    fn run_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>>;
}

