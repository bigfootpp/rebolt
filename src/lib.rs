mod rules;

use rules::Re;
use std::ops::Range;

use regex::Regex;

#[derive(Debug)]
pub struct Match {
    pub name: String,
    pub content: String,
    pub range: Range<usize>,
    pub children: Matches,
}

#[derive(Debug)]
pub struct Matches {
    matches: Vec<Match>,
}

impl Matches {
    pub fn named(&self, name: &str) -> Vec<&Match> {
        self.matches.iter().filter(|m| m.name == name).collect()
    }
}

pub struct Rebolt {
    rules: Vec<Re>,
}

impl Rebolt {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn regex(mut self, name: impl Into<String>, regex_str: &str) -> Self {
        let compiled_str =
            Regex::new(regex_str).expect(&format!("Regular expression not valid: {regex_str}"));
        self.rules.push(Re {
            name: name.into(),
            regex: compiled_str,
        });
        self
    }
    pub fn matches(&self, text: &str) -> Matches {
        let mut result: Vec<Match> = Vec::new();
        for rule in &self.rules {
            let matches = rule.find_matches(text);
            result.extend(matches);
        }
        Matches { matches: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matches() {
        let rebolt = Rebolt::new()
            .regex("year", r"(\d{4})")
            .regex("resolution", r"(\d{3,4}p)");
        let matches = rebolt.matches("The.Matrix.1999.1080p");
        let year = matches.named("year")[0].children.named("year");
        let resolution = matches.named("resolution")[0].children.named("resolution");

        assert_eq!(year[0].content, "1999");
        assert_eq!(resolution[0].content, "1080p");
    }
}
