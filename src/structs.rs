use serde::Deserialize;

#[derive(Deserialize)]
pub struct Metadata {
    pub title: String,
    pub title_title: String,
    pub chapter: u16,
    pub chapter_title: String,
    pub subchapter: String,
    pub subchapter_title: String,
    pub part: u16,
    pub part_title: String,
}

#[derive(Deserialize)]
pub struct Regulation {
    pub title: String,
    pub chapter: u16,
    pub subchapter: String,
    pub part: u16,
    pub section: u16,
    pub section_title: String,
    pub paragraph: Option<String>,
    pub subparagraph: Option<u16>,
    pub item: Option<u16>,
    pub content: String,
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct InputData {
    pub metadata: Vec<Metadata>,
    pub regulations: Vec<Regulation>,
}
