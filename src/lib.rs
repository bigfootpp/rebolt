use regex::Regex;

struct Re {
    name: String,
    regex: Regex,
}

pub struct Rebolt {
    rules: Vec<Re>,
}

impl Rebolt {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn regex(mut self, name: impl Into<String>, regex_str: &str) -> Self {
        let compiled_str =
            Regex::new(regex_str).expect(&format!("Regular expression not valid: {regex_str}"));
        self.rules.push(Re {
            name: name.into(),
            regex: compiled_str,
        });
        self
    }
}
