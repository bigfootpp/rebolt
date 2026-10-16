pub mod rules;
pub mod types;
mod utils;

use std::collections::{HashMap, HashSet};

pub use rules::{Re, Rule};
pub use types::{IdSize, Match, Matches};

use crate::utils::IdGenerator;

fn get_id_by_name(matches: &Vec<Match>) -> HashMap<String, HashSet<IdSize>> {
    let mut map: HashMap<String, HashSet<IdSize>> = HashMap::new();
    for m in matches {
        let mut set = map.remove(&m.name).unwrap_or(HashSet::new());
        set.insert(m.id);
        map.insert(m.name.clone(), set);
    }
    map
}

fn get_id_by_parent_id(matches: &Vec<Match>) -> HashMap<IdSize, HashSet<IdSize>> {
    let mut map: HashMap<IdSize, HashSet<IdSize>> = HashMap::new();
    for m in matches {
        if let Some(id) = &m.parent_id {
            let mut set = map.remove(id).unwrap_or(HashSet::new());
            set.insert(m.id);
            map.insert(*id, set);
        }
    }
    map
}

pub struct Rebolt {
    rules: Vec<Box<dyn Rule>>,
}

impl Rebolt {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }
    pub fn matches(&self, text: &str) -> Matches {
        let mut id_gen = IdGenerator::new();
        let mut result: Vec<Match> = Vec::new();
        for rule in &self.rules {
            let matches = rule.find_matches(text, &mut id_gen);
            result.extend(matches);
        }
        let by_name = get_id_by_name(&result);
        let parent_map: HashMap<IdSize, HashSet<IdSize>> = get_id_by_parent_id(&result);
        let mut order: Vec<IdSize> = result.iter().map(|m| m.id).collect();
        order.sort_unstable();
        Matches {
            matches: result.into_iter().map(|m| (m.id, m)).collect(),
            by_name: by_name,
            by_parent: parent_map,
            order: order,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::Re;

    use super::*;
    #[test]
    fn test_matches() {
        let rebolt = Rebolt::new()
            .rule(Re::new("year", r"(\d{4})"))
            .rule(Re::new("resolution", r"(\d{3,4}p)"));
        let matches = rebolt.matches("The.Matrix.1999.1080p");
        let year = matches.named_children("year");
        let resolution = matches.named_children("resolution");

        assert_eq!(year[0].content, "1999");
        assert_eq!(resolution[0].content, "1080p");
    }

    #[test]
    fn test_named_group_matches() {
        {
            let rebolt = Rebolt::new().rule(
                Re::new("episode_info", r"S(?P<season>\d{2})E(?P<episode>\d{2})").priority(1),
            );
            let matches = rebolt.matches("Doctor.Who.S02E05.720p");
            let parent = matches.named("episode_info")[0];

            assert_eq!(parent.content, "S02E05");
            assert_eq!(parent.range, 11..17);

            let season = matches.named_children("season")[0];
            let episode = matches.named_children("episode")[0];

            assert_eq!(season.content, "02");
            assert_eq!(episode.content, "05");
        }
        {
            let rebolt = Rebolt::new().rule(Re::new(
                "episode_info",
                r"(?:S(?P<season>\d+))?E(?P<episode>\d+)",
            ));
            let matches = rebolt.matches("Doctor.Who.E12");
            let parent = matches.named("episode_info")[0];

            assert_eq!(parent.content, "E12");
            assert_eq!(parent.range, 11..14);

            let season_no_exist = matches.named_children("season").get(0).copied();
            let episode = matches.named_children("episode")[0];

            assert_eq!(episode.content, "12");
            assert!(season_no_exist.is_none());
        }
    }
}
