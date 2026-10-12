use super::{Match, Matches};
use regex::Regex;

pub struct Re {
    pub name: String,
    pub regex: Regex,
}

impl Re {
    pub fn find_matches(&self, text: &str) -> Vec<Match> {
        let mut result: Vec<Match> = Vec::new();

        for m in self.regex.captures_iter(text) {
            let mut children_matches: Vec<Match> = Vec::new();
            let start = if m.len() > 1 { 1 } else { 0 };
            let groups = m.iter().skip(start);
            let names = self.regex.capture_names().skip(start);
            for (name_opt, group_opt) in names.zip(groups) {
                if let Some(group) = group_opt {
                    let name = name_opt.unwrap_or(&self.name);
                    children_matches.push(Match {
                        name: name.to_string(),
                        content: group.as_str().to_string(),
                        range: group.range(),
                        children: Matches {
                            matches: Vec::new(),
                        },
                    });
                }
            }
            result.push(Match {
                name: self.name.clone(),
                content: m.get(0).unwrap().as_str().to_string(),
                range: m.get(0).unwrap().range(),
                children: Matches {
                    matches: children_matches,
                },
            });
        }
        result
    }
}
