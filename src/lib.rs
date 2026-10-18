pub mod resolver;
pub mod rules;
pub mod types;
mod utils;

use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

pub use rules::{Re, Rule};
pub use types::{IdSize, Match, Matches};

use crate::{
    resolver::{Resolver, simple_resolver::SimpleResolver},
    types::ConflictContext,
    utils::IdGenerator,
};

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
    resolver: Box<dyn Resolver>,
}

impl Rebolt {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            resolver: Box::new(SimpleResolver::new()),
        }
    }
    pub fn resolver(mut self, resolver: impl Resolver + 'static) -> Self {
        self.resolver = Box::new(resolver);
        self
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
        let id_map: HashMap<IdSize, Match> = result.into_iter().map(|m| (m.id, m)).collect();

        let mut ctx = ConflictContext::new(id_map, conflict);
        self.resolver.resolve(&mut ctx);
        let final_matches = ctx.into_inner();

        order.sort_unstable();
        Matches {
            matches: final_matches,
            by_name: by_name,
            order: order,
        }
    }
}
