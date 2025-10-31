use interfaces::{Item, Parser, TokenType};
use std::error::Error;
use std::fmt;

pub struct ScriptParser;

#[derive(Debug)]
enum ParseError {
    ParsingFailed,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ParsingFailed => write!(f, "Item parsing failed"),
        }
    }
}

impl Error for ParseError {}

impl ScriptParser {
    pub fn new() -> Self {
        ScriptParser {}
    }

    fn parse_load_plugin(&self, item: &mut Item) -> bool {
        true
    }

    fn parse_constant_macro(&self, item: &mut Item) -> bool {
        true
    }

    fn parse_variable_macro(&self, item: &mut Item) -> bool {
        true
    }

    fn parse_command(&self, item: &mut Item) -> bool {
        true
    }

    fn parse_if_goto(&self, item: &mut Item) -> bool {
        true
    }

    fn parse_label(&self, item: &mut Item) -> bool {
        true
    }

    fn parse_item(&self, item: &mut Item) -> bool {
        println!("\tValidating item {:?}", item);

        match item.token_type {
            TokenType::LoadPlugin {
                name: _,
                rule: _,
                vers: _,
            } => {
                return self.parse_load_plugin(item);
            }
            TokenType::ConstantMacro { name: _, value: _ } => {
                return self.parse_constant_macro(item);
            }
            TokenType::VariableMacro {
                plugin: _,
                command: _,
                args: _,
                name: _,
                value: _,
            } => {
                return self.parse_variable_macro(item);
            }
            TokenType::Command {
                plugin: _,
                command: _,
                args: _,
            } => {
                return self.parse_command(item);
            }
            TokenType::IfGoTo {
                condition: _,
                label: _,
            } => {
                return self.parse_if_goto(item);
            }
            TokenType::Label { label: _ } => {
                return self.parse_label(item);
            }
            _ => {
                println!("Unexpected token type..");
                return false;
            }
        }
    }
}

impl Parser for ScriptParser {
    fn parse_script(&self, inout: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Parsing script ...");
        for item in inout {
            if !self.parse_item(item) {
                return Err(Box::new(ParseError::ParsingFailed));
            }
        }
        Ok(())
    }
}
