use std::cmp;

use crate::{
    resolver::Resolver,
    types::{ConflictContext, ConflictDecision},
};

pub struct SimpleResolver;

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
