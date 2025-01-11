use database_utils::structs::*;
use jsonc_parser::parse_to_serde_value;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::fs;

fn main() -> Result<()> {
    let json_data = fs::read_to_string("input.jsonc").expect("Unable to read input file");
    let data_serde = parse_to_serde_value(&json_data, &Default::default())
        .expect("Failed to parse input file")
        .unwrap();

    let data: InputData = serde_json::from_value(data_serde).expect("Failed to parse input file");

    let metadata_data = &data.metadata;
    let regulations_data = &data.regulations;

    let conn = Connection::open("regulations.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS metadata (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            title_title TEXT NOT NULL,
            chapter INTEGER NOT NULL,
            chapter_title TEXT NOT NULL,
            subchapter TEXT NOT NULL,
            subchapter_title TEXT NOT NULL,
            part INTEGER NOT NULL,
            part_title TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS regulations (
                rowid INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                chapter INTEGER NOT NULL,
                subchapter TEXT NOT NULL,
                part INTEGER NOT NULL,
                section INTEGER NOT NULL,
                section_title TEXT NOT NULL,
                paragraph TEXT,
                subparagraph INTEGER,
                item INTEGER,
                content TEXT NOT NULL,
                image TEXT
            )",
        [],
    )?;

    let mut metadata_stmt = conn.prepare(
        "INSERT INTO metadata (title, title_title, chapter, chapter_title, subchapter, subchapter_title, part, part_title)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )?;

    for metadata_entry in metadata_data {
        metadata_stmt.execute(params![
            metadata_entry.title,
            metadata_entry.title_title,
            metadata_entry.chapter,
            metadata_entry.chapter_title,
            metadata_entry.subchapter,
            metadata_entry.subchapter_title,
            metadata_entry.part,
            metadata_entry.part_title,
        ])?;
    }

    let mut stmt = conn.prepare(
        "INSERT INTO regulations (title, chapter, subchapter, part, section, section_title, paragraph, subparagraph, item, content, image)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    )?;

    for reg in regulations_data {
        let paragraph = reg.paragraph.as_deref();
        let image = reg.image.as_deref();

        stmt.execute(params![
            reg.title,
            reg.chapter,
            reg.subchapter,
            reg.part,
            reg.section,
            reg.section_title,
            paragraph,
            reg.subparagraph,
            reg.item,
            reg.content,
            image
        ])?;
    }

    Ok(())
}
