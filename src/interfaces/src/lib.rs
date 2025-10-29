use std::io;

pub trait Reader {
    fn read_script(&mut self, filepathname: &str, output : &mut Vec<String>) -> Result<usize, io::Error>;
}