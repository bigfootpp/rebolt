pub mod re;
pub mod text;

use super::Match;
use crate::utils::IdGenerator;

pub use re::Re;
pub use text::Text;

pub trait Rule {
    fn get_priority(&self) -> u8;
    fn find_matches(&self, text: &str, id_gen: &mut IdGenerator) -> Vec<Match>;
}
