mod model;

use std::result::Result;
use std::io::Error;

fn main() {
    let f: model::File = parse_json_into_file().unwrap_or_else(|err| {
        panic!("Error trying to parse the JSON file: {:?}", err)
    });

    println!("{:#?}", f);
}

fn parse_json_into_file() -> Result<model::File, Error> {
    // TODO: create some validations on json, for instance: start with -1
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

    return match serde_json::from_str(data) {
        Ok(file) => Ok(file),
        Err(e) => Err(e.into()),
    };
}
