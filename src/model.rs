use serde::Deserialize;
use std::fmt::Display;

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

#[derive(Deserialize, Debug)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

#[derive(Deserialize, Debug)]
pub struct Parameter {
    pub text: String,
    pub location: Location,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Print(Print),
    Str(Str),
    Let(Let),
}

// TODO: We should refactor this Trap into something like: "Error", "RuntimeError"...
#[derive(Debug)]
pub struct Trap {
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum Value {
    Str(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Str(str) => str.to_string(),
        };
        f.write_str(&value)
    }
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

#[derive(Debug, Deserialize)]
pub struct Str {
    pub value: String, // TODO: move this to generic value
    pub location: Location,
}

impl Element for Str {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Deserialize)]
pub struct Let {
    pub name: Parameter,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

impl Element for Let {
    fn location(&self) -> &Location {
        &self.location
    }
}
