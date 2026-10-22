use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

pub type IdSize = usize;

#[derive(Debug)]
pub struct Match {
    pub id: IdSize,
    pub priority: u8,
    pub name: String,
    pub content: String,
    pub range: Range<IdSize>,
    pub parent_id: Option<IdSize>,
    pub child_ids: HashSet<IdSize>,
}

impl Match {
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
    pub fn is_internal(&self) -> bool {
        self.parent_id.is_some() && !self.child_ids.is_empty()
    }
    pub fn is_leaf(&self) -> bool {
        self.child_ids.is_empty()
    }
}

#[derive(Debug)]
pub struct Matches {
    pub matches: HashMap<IdSize, Match>,
    pub by_name: HashMap<String, HashSet<IdSize>>,
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

pub enum ConflictDecision {
    KeepLeft,
    KeepRight,
    Ignore,
}

pub struct ConflictCursor<'a> {
    context: &'a ConflictContext,
    left_id: IdSize,
    right_id: IdSize,
}

pub struct ConflictContext {
    matches: HashMap<IdSize, Match>,
    conflict_set: HashSet<(IdSize, IdSize)>,
    discarded: HashSet<IdSize>,
}

impl ConflictContext {
    pub fn new(matches: HashMap<IdSize, Match>, conflicts: HashSet<(IdSize, IdSize)>) -> Self {
        Self {
            matches,
            conflict_set: conflicts,
            discarded: HashSet::new(),
        }
    }

    pub fn for_each_conflict<F>(&mut self, handler: F)
    where
        F: Fn(&ConflictCursor) -> ConflictDecision,
    {
        for c in self.conflict_set.clone() {
            if self.is_discarded(&c.0) || self.is_discarded(&c.1) {
                continue;
            }
            let cursor = ConflictCursor {
                context: &self,
                left_id: c.0,
                right_id: c.1,
            };
            let decision = handler(&cursor);
            let to_discard = match decision {
                ConflictDecision::KeepLeft => Some(c.1),
                ConflictDecision::KeepRight => Some(c.0),
                ConflictDecision::Ignore => None,
            };
            if let Some(id) = to_discard {
                self.discard(id);
                self.discard_empty_parents(id);
            }
        }
    }

    fn discard_empty_parents(&mut self, id: IdSize) {
        let mut current = self
            .matches
            .get(&id)
            .and_then(|m| m.parent_id)
            .filter(|parent_id| !self.is_discarded(parent_id));

        while let Some(parent_id) = current {
            let Some((parent_parent, mut child_ids)) = self
                .matches
                .get(&parent_id)
                .map(|m| (m.parent_id, m.child_ids.clone()))
            else {
                return;
            };
            child_ids.retain(|child_id| !self.is_discarded(child_id));
            if child_ids.is_empty() {
                self.discard(parent_id);
                current = parent_parent.filter(|id| !self.is_discarded(id));
            } else {
                if let Some(parent) = self.matches.get_mut(&parent_id) {
                    parent.child_ids = child_ids;
                }
                return;
            }
        }
    }

    pub fn get(&self, id: IdSize) -> Option<&Match> {
        if self.is_discarded(&id) {
            None
        } else {
            self.matches.get(&id)
        }
    }

    pub fn into_inner(self) -> HashMap<IdSize, Match> {
        self.matches
            .into_iter()
            .filter(|(id, _)| !self.discarded.contains(id))
            .collect()
    }

    pub fn is_discarded(&self, id: &IdSize) -> bool {
        self.discarded.contains(id)
    }

    pub fn discard(&mut self, id: IdSize) {
        self.discarded.insert(id);
    }
}

impl ConflictCursor<'_> {
    pub fn left(&self) -> &Match {
        self.context.get(self.left_id).unwrap()
    }

    pub fn right(&self) -> &Match {
        self.context.get(self.right_id).unwrap()
    }
}
