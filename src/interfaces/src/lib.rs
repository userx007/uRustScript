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

/*
pub trait PluginInterface {
    fn is_initialized(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn set_params(&mut self, params: &PluginDataSet) -> bool;
    fn get_params(&self) -> PluginDataGet;
    fn get_data(&self) -> &str;
    fn reset_data(&mut self);
    fn do_init(&mut self, user_data: *mut std::ffi::c_void) -> bool;
    fn do_enable(&mut self);
    fn do_dispatch(&mut self, cmd: &str, params: &str) -> bool;
    fn do_cleanup(&mut self);
    fn is_fault_tolerant(&self) -> bool;
    fn is_privileged(&self) -> bool;
}


#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn PluginInterface;
#[no_mangle]
pub extern "C" fn destroy_plugin(ptr: *mut dyn PluginInterface);
*/