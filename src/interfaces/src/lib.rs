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
