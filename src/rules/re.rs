use std::collections::HashSet;

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

        for m in self.regex.captures_iter(text) {
            let mut child_ids = HashSet::new();

            let parent_id = id_gen.next();
            let start = if m.len() > 1 { 1 } else { 0 };

            let groups = m.iter().skip(start);
            let names = self.regex.capture_names().skip(start);

            for (name_opt, group_opt) in names.zip(groups) {
                if let Some(group) = group_opt {
                    let name = name_opt.unwrap_or(&self.name);
                    let id = id_gen.next();
                    result.push(Match {
                        id: id,
                        priority: self.priority,
                        name: name.to_string(),
                        content: group.as_str().to_string(),
                        range: group.range(),
                        child_ids: HashSet::new(),
                        parent_id: Some(parent_id),
                    });

                    child_ids.insert(id);
                }
            }
            result.push(Match {
                id: parent_id,
                priority: self.priority,
                name: self.name.clone(),
                content: m.get(0).unwrap().as_str().to_string(),
                range: m.get(0).unwrap().range(),
                child_ids: child_ids,
                parent_id: None,
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Rebolt;

    use super::*;

    #[test]
    fn matches_multiple_rules() {
        let rebolt = Rebolt::new()
            .rule(Re::new("year", r"(\d{4})"))
            .rule(Re::new("resolution", r"(\d{3,4}p)"));
        let matches = rebolt.matches("The.Matrix.1999.1080p");

        assert_eq!(matches.named_children("year")[0].content, "1999");
        assert_eq!(matches.named_children("resolution")[0].content, "1080p");
    }

    #[test]
    fn matches_named_groups() {
        let rebolt = Rebolt::new()
            .rule(Re::new("episode_info", r"S(?P<season>\d{2})E(?P<episode>\d{2})").priority(1));
        let matches = rebolt.matches("Doctor.Who.S02E05.720p");

        let parent = matches.named("episode_info")[0];
        assert_eq!(parent.content, "S02E05");
        assert_eq!(parent.range, 11..17);

        let season = matches.named_children("season")[0];
        let episode = matches.named_children("episode")[0];
        assert_eq!(season.content, "02");
        assert_eq!(episode.content, "05");
    }

    #[test]
    fn skips_missing_optional_group() {
        let rebolt = Rebolt::new()
            .rule(Re::new("episode_info", r"(?:S(?P<season>\d+))?E(?P<episode>\d+)"));
        let matches = rebolt.matches("Doctor.Who.E12");

        let season = matches.named_children("season");
        let episode = matches.named_children("episode");
        assert!(season.is_empty());
        assert_eq!(episode[0].content, "12");
    }
}
