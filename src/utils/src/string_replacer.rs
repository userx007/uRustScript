use regex::Regex;
use std::collections::HashMap;

pub fn replace_macros(line: &mut String, map: &HashMap<String, String>) -> bool {
    if !line.contains('$') || map.is_empty() {
        return false;
    }

    // Sort keys by length descending (longest first)
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));

    // Join keys into a regex alternation: (XXXX|XXX|...)
    let pattern = keys
        .iter()
        .map(|k| regex::escape(k))
        .collect::<Vec<_>>()
        .join("|");

    let re = Regex::new(&format!(r"\$({})", pattern)).unwrap();

    // Fast path: check if anything matches
    if !re.is_match(line) {
        return false;
    }

    // Replace matches using map
    let replaced = re.replace_all(line, |caps: &regex::Captures| {
        let key = &caps[1];
        map.get(key).map(|v| v.as_str()).unwrap_or("")
    });

    // Only assign back if the line actually changed
    if let std::borrow::Cow::Owned(new_text) = replaced {
        *line = new_text;
        true
    } else {
        false
    }
}
