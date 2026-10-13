pub mod re;

use super::Match;

pub use re::Re;

pub trait Rule {
    fn find_matches(&self, text: &str) -> Vec<Match>;
}
