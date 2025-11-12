use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone)]
pub struct IniParserEx {
    ini_data: HashMap<String, HashMap<String, String>>,
}

impl IniParserEx {
    /// Load an INI file into memory.
    pub fn load(&mut self, filename: &str) -> bool {
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let reader = BufReader::new(file);
        let mut current_section = String::new();

        for mut line in reader.lines().map_while(Result::ok) {
            line = Self::trim(&line);
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            // Section header: [Section]
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }
            // Key=Value pair
            if let Some(pos) = line.find('=') {
                let key = Self::trim(&line[..pos]);
                let value = Self::trim(&line[pos + 1..]);
                self.ini_data
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, value);
            }
        }
        true
    }

    /// Get a value by section/key with optional default and variable substitution.
    pub fn get_value(&self, section: &str, key: &str, default_value: &str, depth: usize) -> String {
        if depth == 0 {
            return default_value.to_string();
        }

        if let Some(section_map) = self.ini_data.get(section) {
            if let Some(value) = section_map.get(key) {
                let mut resolved = value.clone();
                let re = Regex::new(r"\$\{([^}]+)\}").unwrap();

                while let Some(caps) = re.captures(&resolved) {
                    let var_name = &caps[1];
                    let replacement = if let Some(colon_pos) = var_name.find(':') {
                        let var_section = &var_name[..colon_pos];
                        let var_key = &var_name[colon_pos + 1..];
                        self.get_value(var_section, var_key, "", depth - 1)
                    } else {
                        self.get_value(section, var_name, "", depth - 1)
                    };
                    resolved = re.replace(&resolved, replacement.as_str()).to_string();
                }
                return resolved;
            }
        }

        default_value.to_string()
    }

    /// Retrieve a section's key/value pairs.
    pub fn get_section(&self, section: &str) -> Option<HashMap<String, String>> {
        self.ini_data.get(section).cloned()
    }

    /// Retrieve a section with all values resolved recursively.
    pub fn get_resolved_section(
        &self,
        section: &str,
        depth: usize,
    ) -> Option<HashMap<String, String>> {
        let mut result = HashMap::new();
        if let Some(section_map) = self.ini_data.get(section) {
            for key in section_map.keys() {
                let resolved = self.get_value(section, key, "", depth);
                result.insert(key.clone(), resolved);
            }
            return Some(result);
        }
        None
    }

    /// Check if a section exists.
    pub fn section_exists(&self, section: &str) -> bool {
        self.ini_data.contains_key(section)
    }

    /// Helper: trim whitespace.
    fn trim(s: &str) -> String {
        s.trim_matches(|c: char| c.is_whitespace() || c == '\r' || c == '\n')
            .to_string()
    }
}

/*

fn main() {
    let mut parser = IniParserEx::default();

    if parser.load("config.ini") {
        println!("Loaded!");

        let val = parser.get_value("Common", "path", "default", 5);
        println!("Common.path = {}", val);

        if let Some(section) = parser.get_resolved_section("Common", 5) {
            println!("Resolved Common section:");
            for (k, v) in section {
                println!("  {} = {}", k, v);
            }
        }
    } else {
        println!("Failed to load file");
    }
}


*/
