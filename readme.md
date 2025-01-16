# CFR Database

Very fast database for federal regulations applicable to pilots, the FAA AIM, and the pilot / controller glossary.

## What is covered:

1. **Title 14; Chapter 1 - `Federal Aviation Administration, Department of Transportation`**
    - Subchapter A -`Definitions and General Requirements` Parts:
        - 1 - `Definitions and Abbreviations`
        - 5 - `Safety Management Systems`
    - Subchapter C - `Aircraft` Parts:
        - 43 - `Maintenance, Preventive Maintenance, Rebuilding, and Alteration`
        - 48 - `Registration and Marking Requirements for Small Unmanned Aircraft`
    - Subchapter D - `Airmen` Parts:
        - 61 - `Certification: Pilots, Flight Instructors, and Ground Instructors`
        - 63 - `Certification: Flight Crewmembers Other Than Pilots`
        - 65 - `Certification: Airmen Other Than Flight Crewmembers`
        - 67 - `Medical Standards and Certification`
        - 68 - `Requirements for Operating Certain Small Aircraft Without a Medical Certificate`
    - Subchapter E - `Airspace` Parts:
        - 71 - `Designation of Class A, B, C, D, and E Airspace Areas; Air Traffic Service Routes; and Reporting Points`
        - 73 - `Special Use Airspace`
    - Subchapter F - `Air Traffic and General Operating Rules` Parts:
        - 89 - `Remote Identification of Unmanned Aircraft`
        - 91 - `General Operating and Flight Rules`
        - 95 - `IFR Altitudes`
        - 97 - `Standard Instrument Procedures`
        - 103 - `Ultralight Vehicles`
        - 105 - `Parachute Operations`
        - 107 - `Small Unmanned Aircraft Systems`
    - Subchapter G - `Air Carriers and Operators for Compensation or Hire: Certification and Operations` Parts:
        - 110 - `General Requirements`
        - 111 - `Pilot Records Database`
        - 117 - `Flight and Duty Limitations and Rest Requirements: Flightcrew Members`
        - 119 - `Certification: Air Carriers and Commercial Operators`
        - 120 - `Drug and Alcohol Testing Program`
        - 121 - `Operating Requirements: Domestic, Flag, and Supplemental Operations`
        - 125 -
          `Certification and Operations: Aircraft Having a Seating Capacity of 20 or More Passengers or a Maximum Payload Capacity of 6,000 Pounds or More; and Rules Governing Persons on Board Such Aircraft`
        - 135 - `Operating Requirements: Commuter and on Demand Operations and Rules Governing Persons on Board Such Aircraft`
        - 136 - `Commercial Air Tours and National Parks Air Tour Management`
        - 137 - `Agricultural Aircraft Operations`
    - Subchapter H - `Schools and Other Certificated Agencies` Parts:
        - 141
        - 142
2. **Title 49; Subtitle B - `Other Regulations Relating to Transportation`:**
    - Chapter 8 - `National Transportation Safety Board` Part:
        - 830 `Notification and Reporting of Aircraft Accidents or Incidents and Overdue Aircraft, and Preservation of Aircraft Wreckage, Mail, Cargo, and Records`
    - Chapter 12 - `Transportation Security Administration, Department of Homeland Security` Part:
        - 1552 `Flight Training Security Program`
3. AIM (Aeronautical Information Manual)
4. Pilot / Controller Glossary
5. NASA Reporting System

## Tables

The Database is made of 5 main tables:

- FAR Metadata - Stores the names of `chapters`, `subchapters`, `subparts`, `regulation_names` etc.
- AIM Metadata - Stores the names of `chapters`, `subchapters`
- FAR Entries - Stores the actual data for 14 and 49 CFR
- AIM Entries - Stores data for the AIM
- PC Entries - Stores all the terms and definitions in the pilot / controller glossary

The reason for Metadata and Entries being seperated is for simplicity. A user can use the Metadata and sqls sorting to build a fully functional table of content. From here determining what regulation
to obtain is quite simple since the TOC or a search will provide us with everything we need to access the data in the entries table, load into memory and render.

## Building:

I dont own datagrip + I want ai to autocomplete the regulations thus, the input is a json file (jsonc since I'm fancy like that) which is just parsed and the DB is generated from that. Schema is in
the repo `database_scema.json`

```shell
# have a valid  input.database.jsonc
cargo run --bin build_db
# take db
# use db
```

- if a reg is only one paragraph it still needs paragraph id in the input data

## Usage API

```rust
use databse_utils::DatabaseInterface;

/**
Structures that data is parsed into: See `lib.rs`
pub struct RenderStructure {
    id: String,
    title: String,
    paragraphs: Vec<Paragraph>,
}

struct Paragraph {
    paragraph_content: Option<String>,
    subparagraphs: Option<Vec<SubParagraph>>,
}

struct SubParagraph {
    subparagraph_content: Option<String>,
    items: Option<Vec<Item>>,
}

struct Item {
    item_content: String,
}
*/
// let's fetch 14CFR Chapter 1 Subchater A Part 1 Section 3 (FAR 1.3 for you simple folk) and print it to the console 
fn main() -> Result<()> {
    let database = DatabaseInterface::new("regulations.db");

    let far_data: RenderStructure = databse_interface.parse_far_database(14, 1, "A", 1, 3)?;
    print!("{}", render_data.format_as_text());
}
/**
Example output
**§ 1.3 Rules of construction**
(a) In Subchapters A through K of this chapter, unless the context requires otherwise:
(a)(1) Words importing the singular include the plural;
(a)(2) Words importing the plural include the singular; and
(a)(3) Words importing the masculine gender include the feminine.
(b) In Subchapters A through K of this chapter, the word:
(b)(1) Shall  is used in an imperative sense;
(b)(2) May  is used in a permissive sense to state authority or permission to do the act prescribed, and the words “no person may * * *” or “a person may not * * *” mean that no person is required, authorized, or permitted to do the act prescribed; and
(b)(3) Includes  means “includes but is not limited to”
*/
```

### Pilot Controller Glossary Parsing

backend:

```rust
use databse_utils::DatabaseInterface;

#[tauri::command]
pub fn parse_pcg_definition(definition: &str) -> Result<ParsedGlossary, String> {
    let database = DatabaseInterface::new("regulations.db");
    if let Some(result) = database.parse_and_search_glossary(definition) {
        Ok(result)
    } else {
        Err(format!("Failed to parse definition"))
    }
}

```

frontend:

```tsx
import React, {useState} from "react";
import {invoke} from "@tauri-apps/api";
import {FarAimContext} from "@utils/context"

type GlossaryLink = {
    id: number;
    text: string;
}

type ParsedGlossary = {
    string_slices: string[],
    links: [number, GlossaryLink][];
}

type GlossaryLinkProps = {
    to: number;
    text: string;
}

const GlossaryLink: React.FC<GlossaryLinkProps> = (props: GlossaryLinkProps) => {
    const {book, location} = React.useContext(FarAimContext);
    const handleClick = (to: number): void => {
        // update context to reflext page change 
    }
    return (<div onClick={() => {
        handleClick(props.to)
    }}> {props.text} </div>)
}


type GlossaryTextProps = {
    glossary: ParsedGlossary;
}

const GlossaryText: Rect.FC<GlossaryTextProps> = (props: GlossaryTextProps) => {
    const {string_slices, links} = props.glossary;

    return (
        <>
            {string_slices.map((string, index) => (
                <React.Fragment key={index}>
                    {string}
                    {links.filter(([i]) => i === index).map(([_, {id, text}]) => (<GlossaryLink key={id} to={id} text={text}/>))}
                </React.Fragment>
            ))}
        </>
    )
}

const GlossaryPage = () => {
    const {book, page} = React.useContext(FarAimContext);
    const [rawDefinition, setRawDefinition] = React.useState<string>("");
    const [term, setTerm] = React.useState<string>("");
    const [glossary, setGlossary] = React.useState<ParsedGlossary | null>(null);

    const fetchGlossary = async () => {
        try {
            const result: ParsedGlossary = await invoke("parse_pcg_definition", {definition: rawDefinition});
            setGlossary(result);
        } catch (e) {
            console.error("Failed while fetching glossary item:", e);
        }
    }

    React.useEffect(() => {
        // fetch raw definition based off page (id in the table) 
        invoke("get_definition", {id: page}).then((result) => {
            setRawDefinition(result.definition)
            setTerm(result.term)
        }).catch((e) => {
            console.error(e)
        });

    }, [page])

    React.useEffect(() => {
        fetchGlossary().catch((e) => {
            console.error(e)
        })
    }, [rawDefintion])

    return (
        <div className="PC_Glossary">
            <h1>The Pilot Controller Glossary</h1>
            {/*   handle searching for terms, when navigating to desired term, use SQL row ID */}
            {glossary && (
                <>
                    <div>{term}</div>
                    <div className="defintion_render">
                        <GlossaryText glossary={glossary}/>
                    </div>
                </>
            )}
        </div>
    )
}

```