pub mod rules;
pub mod types;

pub use rules::{Re, Rule};
pub use types::{Match, Matches};

pub struct Rebolt {
    rules: Vec<Box<dyn Rule>>,
}

impl Rebolt {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }
    pub fn matches(&self, text: &str) -> Matches {
        let mut result: Vec<Match> = Vec::new();
        for rule in &self.rules {
            let matches = rule.find_matches(text);
            result.extend(matches);
        }
        Matches::new(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::Re;

    use super::*;
    #[test]
    fn test_matches() {
        let rebolt = Rebolt::new()
            .rule(Re::new("year", r"(\d{4})"))
            .rule(Re::new("resolution", r"(\d{3,4}p)"));
        let matches = rebolt.matches("The.Matrix.1999.1080p");
        let year = matches.named("year")[0].children.named("year");
        let resolution = matches.named("resolution")[0].children.named("resolution");

        assert_eq!(year[0].content, "1999");
        assert_eq!(resolution[0].content, "1080p");
    }

    #[test]
    fn test_named_group_matches() {
        {
            let rebolt = Rebolt::new().rule(
                Re::new("episode_info", r"S(?P<season>\d{2})E(?P<episode>\d{2})").priority(1),
            );
            let matches = rebolt.matches("Doctor.Who.S02E05.720p");
            let parent = matches.named("episode_info")[0];

            assert_eq!(parent.content, "S02E05");
            assert_eq!(parent.range, 11..17);

            let season = parent.children.named("season")[0];
            let episode = parent.children.named("episode")[0];

            assert_eq!(season.content, "02");
            assert_eq!(episode.content, "05");
        }
        {
            let rebolt = Rebolt::new().rule(Re::new(
                "episode_info",
                r"(?:S(?P<season>\d+))?E(?P<episode>\d+)",
            ));
            let matches = rebolt.matches("Doctor.Who.E12");
            let parent = matches.named("episode_info")[0];

            assert_eq!(parent.content, "E12");
            assert_eq!(parent.range, 11..14);

            let season_no_exist = parent.children.named("season").get(0).copied();
            let episode = parent.children.named("episode")[0];

            assert_eq!(episode.content, "12");
            assert!(season_no_exist.is_none());
        }
    }
}
