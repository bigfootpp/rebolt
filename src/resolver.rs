pub mod simple_resolver;

use crate::types::ConflictContext;

pub trait Resolver {
    fn resolve(&self, context: &mut ConflictContext);
}
