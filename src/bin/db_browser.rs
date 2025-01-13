use database_utils::structs::*;
use database_utils::DatabaseInterface;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rusqlite::{params, Connection, Params, Result};
use serde::Deserialize;
use std::fs;

fn main() -> Result<()> {
    //let conn = Connection::open("regulations.db")?;
    let databse_interface = DatabaseInterface::new("regulations.db");

    let render_data = databse_interface.parse_far_database(14, 1, "A", 1, 3)?;
    print!("{}", render_data.format_as_text());

    // loop {
    //     println!("\nRegulations Database Editor");
    //     println!("1. Add Regulation");
    //     println!("2. View Regulations");
    //     println!("3. Edit Regulation");
    //     println!("4. Delete Regulation");
    //     println!("5. Search Metadata");
    //     println!("6. Exit");

    //     let choice: usize = Input::new()
    //         .with_prompt("Enter your selection")
    //         .validate_with(|input: &String| {
    //             input
    //                 .parse::<usize>()
    //                 .map(|_| ())
    //                 .map_err(|_| "Enter a valid number")
    //         })
    //         .interact_text()
    //         .unwrap()
    //         .parse()
    //         .unwrap();

    //     match choice {
    //         1 => add_regulation(&conn)?,
    //         2 => view_regulations(&conn)?,
    //         3 => edit_regulation(&conn)?,
    //         4 => delete_regulation(&conn)?,
    //         5 => search_metadata_wrapper(&conn)?,
    //         6 => break,
    //         _ => println!("Invalid choice, please try again!"),
    //     }
    // }

    Ok(())
}

fn add_regulation(conn: &Connection) -> Result<()> {
    let title: u8 = Input::<String>::new()
        .with_prompt("Enter Title (e.g., 14)")
        .interact_text()
        .unwrap()
        .parse()
        .unwrap();
    let part: u16 = Input::<String>::new()
        .with_prompt("Enter Part (e.g., 91)")
        .interact_text()
        .unwrap()
        .parse()
        .unwrap();
    let section: u16 = Input::<String>::new()
        .with_prompt("Enter Section (e.g., 103)")
        .interact_text()
        .unwrap()
        .parse()
        .unwrap();
    let content: String = Input::new()
        .with_prompt("Enter Content")
        .interact_text()
        .unwrap();

    let image_input: String = Input::new()
        .with_prompt("Enter Image in Base64 (or press Enter to skip)")
        .interact_text()
        .unwrap();

    let image: Option<String> = if image_input.is_empty() {
        None
    } else {
        Some(image_input)
    };

    let image = image.as_deref();
    conn.execute(
        "INSERT INTO regulations (title, part, section, content, image) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![title, part, section, content, image],
    )?;

    println!("Regulation added successfully!");
    Ok(())
}

fn view_regulations(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT rowid, title, part, section, content, image FROM regulations")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,            // id
            row.get::<_, u8>(1)?,             // title
            row.get::<_, u16>(2)?,            // part
            row.get::<_, u16>(3)?,            // section
            row.get::<_, String>(4)?,         // content
            row.get::<_, Option<String>>(5)?, // image (optional)
        ))
    })?;

    println!("\nCurrent Regulations:");
    for result in rows {
        let (rowid, title, part, section, content, image) = result?;
        println!(
            "ID: {rowid}, Title: {title}, Part: {part}, Section: {section}, Content: {content}"
        );
        if let Some(img) = image {
            println!("Image: {img}");
        } else {
            println!("No image associated.");
        }
    }

    Ok(())
}

fn edit_regulation(conn: &Connection) -> Result<()> {
    let id: i64 = Input::<String>::new()
        .with_prompt("Enter the ID of the regulation to edit")
        .interact_text()
        .unwrap()
        .parse()
        .unwrap();

    let new_content: String = Input::new()
        .with_prompt("Enter new content")
        .interact_text()
        .unwrap();

    let image_input: String = Input::new()
        .with_prompt("Enter new Image in Base64 (or press Enter to skip)")
        .interact_text()
        .unwrap();

    let image: Option<String> = if image_input.is_empty() {
        None
    } else {
        Some(image_input)
    };

    let updated = conn.execute(
        "UPDATE regulations SET content = ?1, image = ?2 WHERE rowid = ?3",
        params![new_content, image, id],
    )?;

    if updated == 1 {
        println!("Regulation updated successfully!");
    } else {
        println!("No regulation found with that ID.");
    }

    Ok(())
}

fn delete_regulation(conn: &Connection) -> Result<()> {
    let id: i64 = Input::<String>::new()
        .with_prompt("Enter the ID of the regulation to delete")
        .interact_text()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let deleted = conn.execute("DELETE FROM regulations WHERE rowid = ?1", params![id])?;

    if deleted == 1 {
        println!("Regulation deleted successfully!");
    } else {
        println!("No regulation found with that ID.");
    }

    Ok(())
}

fn search_metadata_wrapper(conn: &Connection) -> Result<()> {
    println!("\nSearch Metadata");

    let title: String = Input::<String>::new()
        .with_prompt("Enter Title (e.g., 14)")
        .interact_text()
        .unwrap();

    let chapter: Option<u16> = Input::<String>::new()
        .with_prompt("Enter Chapter (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap()
        .parse()
        .ok();

    let subchapter: String = Input::<String>::new()
        .with_prompt("Enter Subchapter (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();
    let subchapter: Option<String> = if subchapter.is_empty() {
        None
    } else {
        Some(subchapter)
    };

    let part: Option<u16> = Input::<String>::new()
        .with_prompt("Enter Part (optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap()
        .parse()
        .ok();

    match search_metadata(conn, title, chapter, subchapter, part, ReturnTypeArg::ALL)? {
        Some(metadata) => match metadata {
            ReturnType::ALL(metadata) => {
                println!(
                    "Found Metadata: Title: {}, Chapter: {}, Subchapter: {}, Part: {}",
                    metadata.title_title,
                    metadata.chapter_title.unwrap_or("Null".to_string()),
                    metadata.subchapter_title.unwrap_or("Null".to_string()),
                    metadata.part_title.unwrap_or("Null".to_string())
                );
            }
            _ => println!("Didnt search with this term"),
        },
        None => println!("No metadata found with the provided criteria."),
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct OptionalMetadata {
    pub title_title: String,
    pub chapter_title: Option<String>,
    pub subchapter_title: Option<String>,
    pub part_title: Option<String>,
}

enum ReturnTypeArg {
    TITLE,
    CHAPTER,
    SUBCHAPTER,
    PART,
    ALL,
}
enum ReturnType {
    SINGLE(String),
    ALL(OptionalMetadata),
}
fn search_metadata(
    conn: &Connection,
    title: String,
    chapter: Option<u16>,
    subchapter: Option<String>,
    part: Option<u16>,
    return_type: ReturnTypeArg,
) -> Result<Option<ReturnType>> {
    let mut query = "SELECT id, title, title_title, chapter, chapter_title, subchapter, subchapter_title, part, part_title FROM metadata WHERE title = ?".to_string();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(title)];

    if let Some(ch) = chapter {
        query.push_str(" AND chapter = ?");
        params.push(Box::new(ch));
    }

    if let Some(sc) = subchapter {
        query.push_str(" AND subchapter = ?");
        params.push(Box::new(sc));
    }

    if let Some(pt) = part {
        query.push_str(" AND part = ?");
        params.push(Box::new(pt));
    }

    let mut stmt = conn.prepare(&query)?;

    let params_slice: Vec<&dyn rusqlite::ToSql> = params.iter().map(|x| &**x).collect();

    let metadata_iter = stmt.query_map(params_slice.as_slice(), |row| {
        Ok(OptionalMetadata {
            title_title: row.get(2)?,
            chapter_title: row.get(4)?,
            subchapter_title: row.get(6)?,
            part_title: row.get(8)?,
        })
    })?;

    for metadata in metadata_iter {
        return match return_type {
            ReturnTypeArg::TITLE => Ok(Some(ReturnType::SINGLE(metadata?.title_title))),
            ReturnTypeArg::CHAPTER => {
                Ok(Some(ReturnType::SINGLE(metadata?.chapter_title.unwrap())))
            }
            ReturnTypeArg::SUBCHAPTER => Ok(Some(ReturnType::SINGLE(
                metadata?.subchapter_title.unwrap(),
            ))),
            ReturnTypeArg::PART => Ok(Some(ReturnType::SINGLE(metadata?.part_title.unwrap()))),
            ReturnTypeArg::ALL => Ok(Some(ReturnType::ALL(metadata?))),
        };
    }

    Ok(None)
}
