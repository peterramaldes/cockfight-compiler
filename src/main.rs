mod interpreter;
mod model;

use crate::model::*;
use std::io::Error;
use std::process;
use std::result::Result;

fn main() {
    // Parse the file and retrieve the structure
    let file: model::File = parse_json().unwrap_or_else(|err| {
        eprintln!("[ERROR] Trying to parse the json file: {err}");
        process::exit(1);
    });

    // TODO: execute the print thing
    let entrypoint = Box::new(file.expression);
    interpreter::eval(entrypoint);
}

fn parse_json() -> Result<File, Error> {
    // TODO: create some validations on json, for instance: start with -1
    // TODO: read the file on `/var/rinha/source.rinha.json`
    // TODO: return somme pretty error if could happen something strange
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
