use std::{collections::HashSet, ops::Range};

use crate::{Match, Rule, utils::IdGenerator};

pub struct Text {
    pub name: String,
    pub text: String,
    pub priority: u8,
}

impl Text {
    pub fn new(name: impl Into<String>, raw_str: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: raw_str.into(),
            priority: 0,
        }
    }
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

impl Rule for Text {
    fn get_priority(&self) -> u8 {
        self.priority
    }
    fn find_matches(&self, text: &str, id_gen: &mut IdGenerator) -> Vec<Match> {
        let mut result: Vec<Match> = Vec::new();

        let ranges: Vec<Range<usize>> = text
            .match_indices(&self.text)
            .map(|(start, _)| start..start + self.text.len())
            .collect();

        for range in ranges {
            let id = id_gen.next();
            let m = &text[range.clone()];
            result.push(Match {
                id: id,
                priority: self.priority,
                name: self.name.clone(),
                content: m.to_string(),
                range: range,
                child_ids: HashSet::new(),
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
            .rule(Text::new("dub", "VF"))
            .rule(Text::new("multi", "MULTI"));
        let matches = rebolt.matches("The.Matrix.1999.1080p.VF.MULTI");

        println!("{:?}", matches);

        let dub = matches.named("dub")[0];
        let multi = matches.named("multi")[0];
        assert_eq!(dub.content, "VF");
        assert_eq!(multi.content, "MULTI");

        assert_eq!(dub.range, 22..24);
        assert_eq!(multi.range, 25..30)
    }
}
