mod model;

use crate::model::*;
use std::io::Error;
use std::process;
use std::result::Result;

fn main() {
    let file: model::File = parse_json_into_file().unwrap_or_else(|err| {
        eprintln!("[ERROR] Trying to parse the json file: {err}");
        process::exit(1);
    });

    // print_type_of(&file.expression);
    println!("{:#?}", file.expression);

    file.expression.eval();
}

fn parse_json_into_file() -> Result<File, Error> {
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
