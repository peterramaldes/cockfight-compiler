use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File { 
    /// The name of the file that generates the AST
    pub name: String,

    /// The expression containing all the info necessary to compile the program
    pub expression: Expression,

    /// Location containing the filename and where inside the file the 
    /// expression is located. 
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
/// Expression represents which kind of structure the expression represents
pub enum Expression {
    Print { value: Box<Expression>, location: Location },
    Str { value: String, location: Location },
}

trait Evaluator {
    fn eval(&self) -> f64;
}

impl Evaluator for Expression {
    fn eval(&self) -> f64 {
        match self {
            Expression::Print { value: _, location: _ } => {
                panic!("todo: implement print")
            },
            Expression::Str { value: _, location: _ } => {
                panic!("todo: implement print")
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location { 
    /// The begin line locate the for the expression
    pub start: u16,

    /// The end line locate the for the expression
    pub end: u16,

    /// The filename where the expression are located
    pub filename: String,
}
