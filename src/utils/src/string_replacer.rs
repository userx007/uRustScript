use regex::Regex;
use std::collections::HashMap;

/// Efficient replacer that modifies &mut String only if needed
pub struct FastReplacer<'a> {
    re: Regex,
    map: &'a HashMap<String, String>,
}

impl<'a> FastReplacer<'a> {
    /// Build once: compile regex and store map reference
    pub fn new(map: &'a HashMap<String, String>) -> Self {
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort_by(|a, b| b.len().cmp(&a.len()));

        let pattern = keys
            .iter()
            .map(|k| regex::escape(k))
            .collect::<Vec<_>>()
            .join("|");

        let re = Regex::new(&format!(r"\$({})", pattern)).unwrap();
        Self { re, map }
    }

    /// Replace placeholders in `text` *in place*.
    /// Returns `true` if a modification was made, `false` otherwise.
    pub fn replace(&self, text: &mut String) -> bool {
        // Fast path: check if regex matches anything at all
        if !self.re.is_match(text) {
            return false; // nothing to replace
        }

        // Perform replacement
        let replaced = self.re.replace_all(text, |caps: &regex::Captures| {
            let key = &caps[1];
            self.map.get(key).map(|v| v.as_str()).unwrap_or("")
        });

        // Only assign if changed
        if let std::borrow::Cow::Owned(new_str) = replaced {
            *text = new_str;
            true
        } else {
            false
        }
    }
}

/*
fn main() {
    let mut map = HashMap::new();
    map.insert("XXX".to_string(), "ValueXXX".to_string());
    map.insert("XXXX".to_string(), "ValueXXXX".to_string());

    let replacer = FastReplacer::new(&map);

    let mut s1 = String::from("This is $XXXX and also $XXX");
    let mut s2 = String::from("No replacement here");

    if replacer.replace(&mut s1) {
        println!("Modified: {s1}");
    } else {
        println!("Unchanged: {s1}");
    }

    if replacer.replace(&mut s2) {
        println!("Modified: {s2}");
    } else {
        println!("Unchanged: {s2}");
    }
}
*/