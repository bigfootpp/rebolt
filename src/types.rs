#[derive(Debug, PartialEq, Clone)]
pub enum Resolution {
    R720p,
    R1080p,
    R2160p,
    R4k,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedData {
    pub raw_title: String,
    pub title: String,
    pub resolution: Option<Resolution>,
}
