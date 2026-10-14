use std::ops::Range;

#[derive(Debug)]
pub struct Match {
    pub name: String,
    pub content: String,
    pub range: Range<usize>,
    pub children: Matches,
}

#[derive(Debug)]
pub struct Matches {
    matches: Vec<Match>,
}

impl Matches {
    pub fn new(matches: Vec<Match>) -> Self {
        Self { matches: matches }
    }
    pub fn named(&self, name: &str) -> Vec<&Match> {
        self.matches.iter().filter(|m| m.name == name).collect()
    }
}
