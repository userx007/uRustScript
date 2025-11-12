use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn replace_macros(line: &mut String, map: &HashMap<String, String>) -> bool {
    if !line.contains('$') || map.is_empty() {
        return false;
    }

    // Sort keys by length descending (longest first)
    let mut keys: Vec<&String> = map.keys().collect();
    //keys.sort_by(|a, b| b.len().cmp(&a.len()));
    keys.sort_by_key(|b| std::cmp::Reverse(b.len()));

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

pub fn string_to_bool(input: &str, out: &mut bool) -> bool {
    let s = input.trim();
    if s.eq_ignore_ascii_case("true") {
        *out = true;
        true
    } else if s.eq_ignore_ascii_case("false") {
        *out = false;
        true
    } else {
        false
    }
}

/// Compare two version strings (`"1.2.3"`, `"2.0"`, etc.) using a rule like `<`, `<=`, `==`, `>`, `>=`, or `!=`.
pub fn compare_versions(v1: &str, rule: &str, v2: &str) -> bool {
    // Parse versions into vectors of numbers (non-numeric parts treated as 0)
    let parse = |v: &str| -> Vec<u64> {
        v.split('.')
            .map(|s| s.parse::<u64>().unwrap_or(0))
            .collect()
    };

    let a = parse(v1);
    let b = parse(v2);

    // Compare element by element
    let mut ord = Ordering::Equal;
    let len = a.len().max(b.len());

    for i in 0..len {
        let x = *a.get(i).unwrap_or(&0);
        let y = *b.get(i).unwrap_or(&0);
        ord = x.cmp(&y);
        if ord != Ordering::Equal {
            break;
        }
    }

    // Apply the comparison rule
    match rule {
        "<" => ord == Ordering::Less,
        "<=" => ord != Ordering::Greater,
        "==" => ord == Ordering::Equal,
        "!=" => ord != Ordering::Equal,
        ">" => ord == Ordering::Greater,
        ">=" => ord != Ordering::Less,
        _ => panic!("Invalid comparison operator: {}", rule),
    }
}
