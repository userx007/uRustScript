pub type ParamsSet = std::collections::HashMap<String, String>;
pub type ParamsGet = std::collections::HashMap<String, String>;

pub trait PluginInterface {
    fn is_initialized(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn set_params(&mut self, params: &ParamsSet) -> bool;
    fn get_params(&self, params: &mut ParamsGet);
    fn do_dispatch(&mut self, cmd: &str, args: &str) -> bool;
    fn reset_data(&mut self);
    fn get_data(&self) -> &str;
}
