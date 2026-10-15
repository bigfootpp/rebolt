use regex::Regex;

use super::Rule;
use crate::{Match, utils::IdGenerator};

pub struct Re {
    pub name: String,
    pub regex: Regex,
    pub priority: u8,
}

impl Re {
    pub fn new(name: impl Into<String>, regex_str: &str) -> Self {
        let compiled_regex =
            Regex::new(regex_str).expect(&format!("Regular expression not valid: {regex_str}"));
        Self {
            name: name.into(),
            regex: compiled_regex,
            priority: 0,
        }
    }
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

impl Rule for Re {
    fn get_priority(&self) -> u8 {
        self.priority
    }
    fn find_matches(&self, text: &str, id_gen: &mut IdGenerator) -> Vec<Match> {
        let mut result: Vec<Match> = Vec::new();
        let parent_id = id_gen.next();

        for m in self.regex.captures_iter(text) {
            let start = if m.len() > 1 { 1 } else { 0 };
            let groups = m.iter().skip(start);
            let names = self.regex.capture_names().skip(start);
            for (name_opt, group_opt) in names.zip(groups) {
                if let Some(group) = group_opt {
                    let name = name_opt.unwrap_or(&self.name);
                    result.push(Match {
                        id: id_gen.next(),
                        name: name.to_string(),
                        content: group.as_str().to_string(),
                        range: group.range(),
                        parent_id: Some(parent_id),
                    });
                }
            }
            result.push(Match {
                id: parent_id,
                name: self.name.clone(),
                content: m.get(0).unwrap().as_str().to_string(),
                range: m.get(0).unwrap().range(),
                parent_id: None,
            });
        }
        result
    }
}
