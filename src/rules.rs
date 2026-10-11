use super::Match;
use regex::Regex;

pub struct Re {
    pub name: String,
    pub regex: Regex,
}

impl Re {
    pub fn find_matches(&self, text: &str) -> Vec<Match> {
        let mut result: Vec<Match> = Vec::new();

        for m in self.regex.captures_iter(text) {
            let start = if m.len() > 1 { 1 } else { 0 };
            for group in m.iter().skip(start).flatten() {
                result.push(Match {
                    name: self.name.clone(),
                    content: group.as_str().to_string(),
                    range: group.range(),
                });
            }
        }
        result
    }
}
