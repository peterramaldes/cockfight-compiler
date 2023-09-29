use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct File { 
    name: String,
    expression: String, // TODO: move to some new struct
    location: String, // TODO: move to some new struct
}

fn main() {
    let _ = parse_json_into_file();
}

fn parse_json_into_file() -> Result<()> {
    // TODO: read the file on `/var/rinha/source.rinha.json`
    let data = r#"
        {
            "name": "ata.rinha",
            "expression": "",
            "location": "",
        }"#;

    let f: File = serde_json::from_str(data).unwrap_or_else(|err| {
        panic!("Error trying to parse the JSON file: {:?}", err)
    });

    println!("{:#?}", f);

    println!("Name: {}, Expression: {}, Location: {}", f.name, f.expression, f.location);

    Ok(())
}
