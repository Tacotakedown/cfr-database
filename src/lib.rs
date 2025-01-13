pub mod structs;

use rusqlite::{params, Connection, Error};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RenderStructure {
    id: String,
    title: String,
    paragraphs: Vec<Paragraph>,
}

impl RenderStructure {
    pub fn format_as_text(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("**{}{}**\n", self.id, self.title));

        for (i, paragraph) in self.paragraphs.iter().enumerate() {
            let paragraph_letter = (b'a' + i as u8) as char;

            if let Some(p_content) = &paragraph.paragraph_content {
                output.push_str(&format!("({}) {}\n", paragraph_letter, p_content));
            }

            if let Some(subparagraphs) = &paragraph.subparagraphs {
                for subparagraph in subparagraphs {
                    if let Some(items) = &subparagraph.items {
                        if let Some(sp_content) = &subparagraph.subparagraph_content {
                            for item in items {
                                output.push_str(&format!(
                                    "({}){} {}\n",
                                    paragraph_letter,
                                    format!("({})", sp_content),
                                    item.item_content
                                ));
                            }
                        }
                    }
                }
            }
        }

        output
    }
}

#[derive(Debug, Clone)]
struct Paragraph {
    paragraph_content: Option<String>,
    subparagraphs: Option<Vec<SubParagraph>>,
}

#[derive(Debug, Clone)]
struct SubParagraph {
    subparagraph_content: Option<String>,
    items: Option<Vec<Item>>,
}

#[derive(Debug, Clone)]
struct Item {
    item_content: String,
}

pub struct DatabaseInterface {
    connection: Connection,
}

impl DatabaseInterface {
    pub fn new(database_file: &str) -> Self {
        Self {
            connection: Connection::open(database_file)
                .expect("Failed to open a connection with the database file"),
        }
    }

    pub fn parse_far_database(
        &self,
        title: u16,
        chapter: u16,
        subchapter: &str,
        part: u16,
        section: u16,
    ) -> Result<RenderStructure, Error> {
        let mut fetch_statement = self.connection.prepare(
            "SELECT * FROM far_entries
            WHERE title = ?1 AND chapter = ?2 AND subchapter = ?3 AND part = ?4 AND section = ?5
            ORDER BY paragraph, subparagraph NULLS FIRST",
        )?;

        let mut rows = fetch_statement.query(params![title, chapter, subchapter, part, section])?;

        let mut render_structure = RenderStructure {
            id: format!("§ {}.{} ", part, section),
            title: String::new(),
            paragraphs: Vec::new(),
        };

        let mut paragraphs_map: HashMap<String, Paragraph> = HashMap::new();

        while let Some(row) = rows.next()? {
            let paragraph_key: Option<String> = row.get("paragraph")?;
            let subparagraph_key: Option<i32> = row.get("subparagraph")?;
            let content: String = row.get("content")?;
            let section_title: String = row.get("section_title")?;

            if render_structure.title.is_empty() {
                render_structure.title = section_title;
            }

            if let Some(p_key) = paragraph_key.clone() {
                let paragraph = paragraphs_map.entry(p_key).or_insert_with(|| Paragraph {
                    paragraph_content: None,
                    subparagraphs: Some(Vec::new()),
                });

                match subparagraph_key {
                    None => {
                        paragraph.paragraph_content = Some(content);
                    }
                    Some(sp_key) => {
                        let subparagraphs = paragraph.subparagraphs.get_or_insert_with(Vec::new);

                        if let Some(existing_sp) = subparagraphs.iter_mut().find(|sp| {
                            sp.subparagraph_content.as_ref() == Some(&sp_key.to_string())
                        }) {
                            existing_sp.items = Some(vec![Item {
                                item_content: content,
                            }]);
                        } else {
                            subparagraphs.push(SubParagraph {
                                subparagraph_content: Some(sp_key.to_string()),
                                items: Some(vec![Item {
                                    item_content: content,
                                }]),
                            });
                        }
                    }
                }
            }
        }

        let mut sorted_paragraphs: Vec<_> = paragraphs_map.into_iter().collect();
        sorted_paragraphs.sort_by(|(a, _), (b, _)| a.cmp(b));
        render_structure.paragraphs = sorted_paragraphs.into_iter().map(|(_, p)| p).collect();

        Ok(render_structure)
    }
}
