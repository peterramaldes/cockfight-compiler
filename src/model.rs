use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

/// An element. It can be a declaration, or a term.
pub trait Element {
    fn location(&self) -> &Location;
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Print(Print),
}

#[derive(Debug, Deserialize)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

impl Element for Print {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}
