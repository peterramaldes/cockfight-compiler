mod interpreter;
mod model;

use crate::model::*;
use std::result::Result;

fn main() -> Result<(), Trap> {
    let file = parse_json()?;

    let entrypoint = Box::new(file.expression);
    let _ = interpreter::eval(entrypoint);

    Ok(())
}

fn parse_json() -> Result<File, Trap> {
    // TODO: create some validations on json, for instance: start with -1
    // TODO: return somme pretty error if could happen something strange

    let path = std::env::args()
        .nth(1)
        .unwrap_or("/var/rinha/source.rinha.json".to_string());

    let file = std::fs::read_to_string(&path).expect(&format!("failed to read file at {}", &path));

    return match serde_json::from_str(&file) {
        Ok(file) => Ok(file),
        Err(e) => Err(Trap {
            message: format!("error trying to parse the json: {}", e),
        }),
    };
}
