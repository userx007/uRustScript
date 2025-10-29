use interfaces::Reader;

pub struct ScriptReader;

impl ScriptReader {
    pub fn new() -> Self {
        ScriptReader
    }
}

impl Reader for ScriptReader {
     fn read_script(&self, name : &str) {
        println!("Reading: {}", name);
     }
}

