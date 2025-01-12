use database_utils::structs::*;
use jsonc_parser::parse_to_serde_value;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use std::fs;

fn main() -> Result<()> {
    let json_data = fs::read_to_string("input.database.jsonc").expect("Unable to read input file");
    let data_serde = parse_to_serde_value(&json_data, &Default::default())
        .expect("Failed to parse input file")
        .unwrap();

    let data: InputData = serde_json::from_value(data_serde).expect("Failed to parse input file");

    let metadata_data = &data.metadata;
    let regulations_data = &data.regulations;

    let conn = Connection::open("regulations.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS far_metadata (
            id INTEGER PRIMARY KEY,
            title INTEGER NOT NULL,
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
        "CREATE TABLE IF NOT EXISTS aim_metadata (
            id INTEGER PRIMARY KEY,
            chapter INTEGER NOT NULL,
            chapter_title TEXT NOT NULL,
            section INTEGER NOT NULL,
            section_title TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS far_entries (
                rowid INTEGER PRIMARY KEY,
                title INTEGER NOT NULL,
                chapter INTEGER NOT NULL,
                subchapter TEXT NOT NULL,
                part INTEGER NOT NULL,
                section INTEGER NOT NULL,
                section_title TEXT NOT NULL,
                paragraph TEXT,
                subparagraph INTEGER,
                item INTEGER,
                content TEXT NOT NULL
            )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS aim_entries (
                rowid INTEGER PRIMARY KEY,
                chapter INTEGER NOT NULL,
                section INTEGER NOT NULL,
                topic INTEGER NOT NULL,
                topic_title TEXT NOT NULL,
                paragraph TEXT,
                subparagraph INTEGER,
                item INTEGER,
                content TEXT NOT NULL,
                image TEXT
            )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pcg_entries (
                rowid INTEGER PRIMARY KEY,
                term TEXT NOT NULL,
                definition TEXT NOT NULL
            )",
        [],
    )?;

    let mut far_metadata_stmt = conn.prepare(
        "INSERT INTO far_metadata (title, title_title, chapter, chapter_title, subchapter, subchapter_title, part, part_title)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    )?;

    let mut aim_metadata_stmt = conn.prepare(
        "INSERT INTO aim_metadata (chapter, chapter_title, section, section_title) VALUES (?1, ?2, ?3, ?4)",
    )?;

    for metadata_entry in metadata_data {
        for far_metadata_entry in &metadata_entry.far_metadata {
            far_metadata_stmt.execute(params![
                far_metadata_entry.title,
                far_metadata_entry.title_title,
                far_metadata_entry.chapter,
                far_metadata_entry.chapter_title,
                far_metadata_entry.subchapter,
                far_metadata_entry.subchapter_title,
                far_metadata_entry.part,
                far_metadata_entry.part_title,
            ])?;
        }
        for aim_metadata_entry in &metadata_entry.aim_metadata {
            aim_metadata_stmt.execute(params![
                aim_metadata_entry.chapter,
                aim_metadata_entry.chapter_title,
                aim_metadata_entry.section,
                aim_metadata_entry.section_title,
            ])?;
        }
    }

    let mut far_entry_stmt = conn.prepare(
        "INSERT INTO far_entries (title, chapter, subchapter, part, section, section_title, paragraph, subparagraph, item, content)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
    )?;

    let mut aim_entry_stmt = conn.prepare(
        "INSERT INTO aim_entries (chapter, section, topic, topic_title, paragraph, subparagraph, item, content, image)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    let mut pcg_entry_stmt =
        conn.prepare("INSERT INTO pcg_entries (term, definition) VALUES (?1, ?2)")?;

    for reg in regulations_data {
        for far_regulation in &reg.far_entry {
            let paragraph = far_regulation.paragraph.as_deref();

            far_entry_stmt.execute(params![
                far_regulation.title,
                far_regulation.chapter,
                far_regulation.subchapter,
                far_regulation.part,
                far_regulation.section,
                far_regulation.section_title,
                paragraph,
                far_regulation.subparagraph,
                far_regulation.item,
                far_regulation.content,
            ])?;
        }

        for aim_entry in &reg.aim_entry {
            let paragraph = aim_entry.paragraph.as_deref();
            let image = aim_entry.image.as_deref();
            aim_entry_stmt.execute(params![
                aim_entry.chapter,
                aim_entry.section,
                aim_entry.topic,
                aim_entry.topic_title,
                paragraph,
                aim_entry.subparagraph,
                aim_entry.item,
                aim_entry.content,
                image,
            ])?;
        }

        for pcg_entry in &reg.pc_entry {
            pcg_entry_stmt.execute(params![pcg_entry.term, pcg_entry.definition,])?;
        }
    }

    Ok(())
}
