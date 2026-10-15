use std::ops::Range;

#[derive(Debug)]
pub struct Match {
    pub id: usize,
    pub name: String,
    pub content: String,
    pub range: Range<usize>,
    pub parent_id: Option<usize>,
}

#[derive(Debug)]
pub struct Matches {
    pub matches: Vec<Match>,
    pub order: Vec<usize>,
}

impl Matches {
    pub fn named(&self, name: &str) -> Vec<&Match> {
        self.matches.iter().filter(|m| m.name == name).collect()
    }
}
