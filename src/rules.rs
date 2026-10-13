pub mod re;

use super::Match;

pub use re::Re;

pub trait Rule {
    fn get_priority(&self) -> u8;
    fn find_matches(&self, text: &str) -> Vec<Match>;
}
