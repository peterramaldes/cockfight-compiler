mod model;

use serde_json::Result;

fn main() {
    let _ = parse_json_into_file();
}

fn parse_json_into_file() -> Result<()> {
    // TODO: read the file on `/var/rinha/source.rinha.json`
    let data = r#"
        {
          "name": "print.rinha",
          "expression": {
            "kind": "Print",
            "value": {
              "kind": "Str",
              "value": "Hello world",
              "location": {
                "start": 7,
                "end": 20,
                "filename": "print.rinha"
              }
            },
            "location": {
              "start": 0,
              "end": 21,
              "filename": "print.rinha"
            }
          },
          "location": {
            "start": 0,
            "end": 21,
            "filename": "print.rinha"
          }
        }"#;

    let f: model::File = serde_json::from_str(data).unwrap_or_else(|err| {
        panic!("Error trying to parse the JSON file: {:?}", err)
    });

    println!("{:#?}", f);

    Ok(())
}
