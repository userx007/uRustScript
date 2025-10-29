use readers::reader_script::ScriptReader;
use interfaces::Reader;


fn main() {
    let reader = ScriptReader::new();
    reader.read_script("script.txt");
}