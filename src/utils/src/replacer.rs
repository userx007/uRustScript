use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;

/// Fast replacer with minimal allocations
pub struct FastReplacer<'a> {
    re: Regex,
    map: &'a HashMap<String, String>,
}

impl<'a> FastReplacer<'a> {
    /// Call-once: prepare regex and store map reference
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

    /// Call-repetitive: replace placeholders efficiently
    pub fn replace<'b>(&self, text: &'b str) -> Cow<'b, str> {
        self.re.replace_all(text, |caps: &regex::Captures| {
            let key = &caps[1];
            self.map.get(key).map(|v| v.as_str()).unwrap_or("")
        })
    }
}


/*
fn main() {
    let mut map = HashMap::new();
    map.insert("XXX".to_string(), "ValueXXX".to_string());
    map.insert("XXXX".to_string(), "ValueXXXX".to_string());

    let replacer = FastReplacer::new(&map);

    let texts = [
        "This is $XXXX and also $XXX",
        "No replacement here",
        "Multiple $XXX $XXXX $XXX",
    ];

    for text in &texts {
        let replaced = replacer.replace(text);
        println!("{}", replaced);
    }
}
*/
