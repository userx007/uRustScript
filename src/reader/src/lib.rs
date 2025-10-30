use interfaces::Reader;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ScriptReader;

impl ScriptReader {
    pub fn new() -> Self {
        ScriptReader {}
    }
}

impl Reader for ScriptReader {
    fn read_script(
        &self,
        filepathname: &str,
        output: &mut Vec<String>,
    ) -> Result<usize, Box<dyn Error>> {
        println!("Reading script: {}", filepathname);
        let file = File::open(filepathname)?;
        let reader = BufReader::new(file);

        let mut in_block_comment = false;

        for line in reader.lines().filter_map(Result::ok) {
            let trimmed = line.trim();

            if in_block_comment {
                if trimmed.ends_with("!--") {
                    in_block_comment = false; // end of block comment
                }
                continue; // skip all lines inside block comment
            } else if trimmed.starts_with("---") {
                in_block_comment = true; // start of block comment
                continue;
            }

            // Skip normal line comments starting with #
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let valid_line = trimmed.to_string();
            // Split only once on '#' to remove the comment at the end of line
            let left = valid_line
                .split_once('#')
                .map(|(left, _)| left.trim()) // take the left part, trim again if needed
                .unwrap_or(&valid_line);

            output.push(left.to_string());
        }
        Ok(output.len())
    }
}
