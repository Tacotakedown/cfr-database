use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FarMetadata {
    pub title: String,
    pub title_title: String,
    pub chapter: u16,
    pub chapter_title: String,
    pub subchapter: String,
    pub subchapter_title: String,
    pub part: u16,
    pub part_title: String,
}

#[derive(Deserialize, Debug)]
pub struct AimMetadata {
    pub chapter: u16,
    pub chapter_title: String,
    pub section: String,
    pub section_title: String,
}

#[derive(Debug, Deserialize)]
pub struct FarEntry {
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
}

#[derive(Debug, Deserialize)]
pub struct AimEntry {
    pub chapter: u16,
    pub section: u16,
    pub topic: u16,
    pub topic_title: String,
    pub paragraph: Option<String>,
    pub subparagraph: Option<u16>,
    pub item: Option<u16>,
    pub content: String,
    pub image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PilotControllerGlossaryEntry {
    pub term: String,
    pub definition: String,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub far_metadata: Vec<FarMetadata>,
    pub aim_metadata: Vec<AimMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct Entries {
    pub far_metadata: Vec<FarEntry>,
    pub aim_metadata: Vec<AimEntry>,
    pub pc_metadata: Vec<PilotControllerGlossaryEntry>,
}

#[derive(Debug, Deserialize)]
pub struct InputData {
    pub metadata: Vec<Metadata>,
    pub regulations: Vec<Entries>,
}
