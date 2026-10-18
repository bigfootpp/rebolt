pub mod resolver;
pub mod rules;
pub mod types;
mod utils;

use std::{
    cmp,
    collections::{HashMap, HashSet},
    ops::Range,
};

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

fn overlaps<T: PartialOrd>(r1: &Range<T>, r2: &Range<T>) -> bool {
    r1.start < r2.end && r2.start < r1.end
}

fn find_conflict(matches: &Vec<Match>) -> HashSet<(IdSize, IdSize)> {
    let mut sorted_match: Vec<&Match> = matches.iter().collect();
    sorted_match.sort_by_key(|m| (m.range.start, m.range.end));
    let mut conflict: HashSet<(IdSize, IdSize)> = HashSet::new();
    let iterator = sorted_match.iter().filter(|m| m.is_leaf());
    for (i, m) in iterator.clone().enumerate() {
        for m2 in iterator.clone().skip(i + 1) {
            if overlaps(&m.range, &m2.range) {
                conflict.insert((m.id, m2.id));
            } else {
                break;
            }
        }
    }
    conflict
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
        let mut order: Vec<IdSize> = result.iter().map(|m| m.id).collect();
        let conflict: HashSet<(IdSize, IdSize)> = find_conflict(&result);
        let mut id_map: HashMap<IdSize, Match> = result.into_iter().map(|m| (m.id, m)).collect();

        let mut final_matches: HashMap<IdSize, Match> = HashMap::new();

        for (c1, c2) in conflict {
            let m1 = id_map.remove(&c1).unwrap();
            let m2 = id_map.remove(&c2).unwrap();

            let best;
            if m1.priority == m2.priority {
                best = cmp::max_by_key(m1, m2, |m| m.range.end - m.range.start)
            } else {
                best = cmp::max_by_key(m1, m2, |m| m.priority)
            }

            final_matches.insert(best.id, best);
        }
        order.sort_unstable();
        Matches {
            matches: final_matches,
            by_name: by_name,
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
