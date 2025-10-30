use std::error::Error;

pub trait Reader {
    fn read_script(&self, filepathname: &str, output : &mut Vec<String>) -> Result<usize, Box<dyn Error>>;
}

pub trait Validator {
    fn validate_script(&self, input : &Vec<String>) -> Result<(), Box<dyn Error>>;
}