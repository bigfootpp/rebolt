use std::ops::Range;

pub use regex::Regex;

fn find_all_matches(re: &Regex, text: &str) -> Vec<Match> {
    let mut result = Vec::new();

    for caps in re.captures_iter(text) {
        let start = if caps.len() > 1 { 1 } else { 0 };

        for m in caps.iter().skip(start).flatten() {
            result.push(Match {
                range: m.range(),
                content: m.as_str().to_string(),
            });
        }
    }

    result
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub regex: Regex,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub range: Range<usize>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub results: Vec<Match>,
}

pub struct Engine {
    rules: Vec<Rule>,
}

impl Engine {
    pub fn matches(&self, text: &str) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();
        for rule in &self.rules {
            let matches = find_all_matches(&rule.regex, text);
            result.push(Token { results: matches });
        }
        result
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_engine() {}
}
