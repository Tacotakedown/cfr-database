use database_utils::structs::*;
use rusqlite::{Connection, Result};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, Debug)]
struct TocEntry {
    title: String,
    children: BTreeMap<String, TocEntry>,
    regulation_id: Option<i64>,
    content_preview: Option<String>,
}

impl TocEntry {
    fn new(title: String) -> Self {
        TocEntry {
            title,
            children: BTreeMap::new(),
            regulation_id: None,
            content_preview: None,
        }
    }

    fn new_with_content(title: String, regulation_id: i64, content: String) -> Self {
        TocEntry {
            title,
            children: BTreeMap::new(),
            regulation_id: Some(regulation_id),
            content_preview: Some(content.chars().take(100).collect()),
        }
    }
}

fn build_table_of_contents(conn: &Connection) -> Result<TocEntry> {
    let mut root = TocEntry::new("FAA Regulations".to_string());

    let mut stmt = conn.prepare(
        "SELECT
            rowid,
            title,
            chapter,
            subchapter,
            part,
            section,
            section_title,
            paragraph,
            subparagraph,
            item,
            content
         FROM regulations
         ORDER BY
            title,
            chapter,
            subchapter,
            part,
            section,
            paragraph,
            subparagraph,
            item",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, Option<String>>(7)?,
            row.get::<_, Option<i64>>(8)?,
            row.get::<_, Option<i64>>(9)?,
            row.get::<_, String>(10)?,
        ))
    })?;

    for result in rows {
        let (
            id,
            title,
            chapter,
            subchapter,
            part,
            section,
            section_title,
            paragraph,
            subparagraph,
            item,
            content,
        ) = result?;

        let title_key = format!("Title {}", title);
        let title_entry = root
            .children
            .entry(title_key.clone())
            .or_insert_with(|| TocEntry::new(title_key));

        let chapter_key = format!("Chapter {}", chapter);
        let chapter_entry = title_entry
            .children
            .entry(chapter_key.clone())
            .or_insert_with(|| TocEntry::new(chapter_key));

        let subchapter_key = format!("Subchapter {}", subchapter);
        let subchapter_entry = chapter_entry
            .children
            .entry(subchapter_key.clone())
            .or_insert_with(|| TocEntry::new(subchapter_key));

        let part_key = format!("Part {}", part);
        let part_entry = subchapter_entry
            .children
            .entry(part_key.clone())
            .or_insert_with(|| TocEntry::new(part_key));

        let section_key = format!("Section {}", section);
        let section_entry = part_entry
            .children
            .entry(section_key.clone())
            .or_insert_with(|| {
                TocEntry::new_with_content(
                    format!("{} - {}", section_key, section_title),
                    id,
                    content.clone(),
                )
            });

        if let Some(para) = paragraph {
            let para_key = format!("Paragraph {}", para);
            let para_entry = section_entry
                .children
                .entry(para_key.clone())
                .or_insert_with(|| TocEntry::new_with_content(para_key, id, content.clone()));

            if let Some(subpara) = subparagraph {
                let subpara_key = format!("Subparagraph {}", subpara);
                let subpara_entry = para_entry
                    .children
                    .entry(subpara_key.clone())
                    .or_insert_with(|| {
                        TocEntry::new_with_content(subpara_key, id, content.clone())
                    });

                if let Some(item_num) = item {
                    let item_key = format!("Item {}", item_num);
                    subpara_entry
                        .children
                        .entry(item_key.clone())
                        .or_insert_with(|| {
                            TocEntry::new_with_content(item_key, id, content.clone())
                        });
                }
            }
        }
    }

    Ok(root)
}

fn print_toc(entry: &TocEntry, depth: usize) {
    println!("{}{}", "  ".repeat(depth), entry.title);
    if let Some(preview) = &entry.content_preview {
        println!("{} Preview: {}", "  ".repeat(depth + 1), preview);
    }
    for child in entry.children.values() {
        print_toc(child, depth + 1);
    }
}

fn main() -> Result<()> {
    let conn = Connection::open("regulations.db")?;

    let toc = build_table_of_contents(&conn)?;

    print_toc(&toc, 0);

    Ok(())
}
