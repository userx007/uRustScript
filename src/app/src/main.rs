use readers::reader_script::ScriptReader;
use interfaces::Reader;


fn main() {
    let mut reader = ScriptReader::new();
    let mut lines = Vec::new();

    match reader.read_script("script.txt", &mut lines) {
        Ok(nr_lines) => {
            println!("Succeeded to read {} lines..", nr_lines);
            for line in lines {
                println!("{}", line);
            }

        }
        Err(err) => {
            println!("Failed to read from file, error: {}", err);
        }
    }
}