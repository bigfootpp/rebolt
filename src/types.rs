use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

pub type IdSize = usize;

#[derive(Debug)]
pub struct Match {
    pub id: IdSize,
    pub name: String,
    pub content: String,
    pub range: Range<IdSize>,
    pub parent_id: Option<IdSize>,
}

#[derive(Debug)]
pub struct Matches {
    pub matches: HashMap<IdSize, Match>,
    pub by_name: HashMap<String, HashSet<IdSize>>,
    pub by_parent: HashMap<IdSize, HashSet<IdSize>>,
    pub order: Vec<IdSize>,
}

impl Matches {
    pub fn named(&self, name: &str) -> Vec<&Match> {
        let Some(id_set) = self.by_name.get(name) else {
            return Vec::new();
        };

        id_set
            .iter()
            .filter_map(|id| self.matches.get(id))
            .filter(|m| m.parent_id.is_none())
            .collect()
    }
    pub fn named_children(&self, name: &str) -> Vec<&Match> {
        let Some(id_set) = self.by_name.get(name) else {
            return Vec::new();
        };

        id_set
            .iter()
            .filter_map(|id| self.matches.get(id))
            .filter(|m| m.parent_id.is_some())
            .collect()
    }
}
