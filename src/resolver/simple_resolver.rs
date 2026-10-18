use std::cmp;

use crate::{
    resolver::Resolver,
    types::{ConflictContext, ConflictDecision},
};

pub struct SimpleResolver;

impl SimpleResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Resolver for SimpleResolver {
    fn resolve(&self, context: &mut ConflictContext) {
        context.for_each_conflict(|c| {
            let left = c.left();
            let right = c.right();

            let best;
            if right.priority == left.priority {
                best = cmp::max_by_key(right, left, |m| m.range.end - m.range.start)
            } else {
                best = cmp::max_by_key(right, left, |m| m.priority)
            }

            if best.id == left.id {
                ConflictDecision::KeepLeft
            } else {
                ConflictDecision::KeepRight
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::{ConflictContext, Match};

    fn leaf(id: usize, priority: u8, range: std::ops::Range<usize>) -> Match {
        Match {
            id,
            priority,
            name: format!("m{id}"),
            content: String::new(),
            range,
            parent_id: None,
            child_ids: Default::default(),
        }
    }

    fn resolved(left: Match, right: Match) -> ConflictContext {
        let (l, r) = (left.id, right.id);
        ConflictContext::new(
            [(l, left), (r, right)].into_iter().collect(),
            [(l, r)].into(),
        )
    }

    #[test]
    fn keeps_higher_priority() {
        let mut ctx = resolved(leaf(0, 5, 0..5), leaf(1, 9, 0..5));
        SimpleResolver::new().resolve(&mut ctx);
        assert!(ctx.get(0).is_none());
        assert!(ctx.get(1).is_some());
    }

    #[test]
    fn keeps_widest_on_equal_priority() {
        let mut ctx = resolved(leaf(0, 5, 2..10), leaf(1, 5, 0..9));
        SimpleResolver::new().resolve(&mut ctx);
        assert!(ctx.get(0).is_none());
        assert!(ctx.get(1).is_some());
    }

    #[test]
    fn keeps_left_when_left_is_best() {
        let mut ctx = resolved(leaf(0, 9, 0..5), leaf(1, 1, 0..5));
        SimpleResolver::new().resolve(&mut ctx);
        assert!(ctx.get(0).is_some());
        assert!(ctx.get(1).is_none());
    }
}
