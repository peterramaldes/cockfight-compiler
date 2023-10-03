use crate::model::*;

pub struct Trap {
    pub message: String,
}

pub fn eval(term: Box<Term>) -> Result<Value, Trap> {
    match *term {
        Term::Print(print) => eval_print(print),
        Term::Str(val) => eval_str(val),
    }
}

pub fn eval_print(print: Print) -> Result<Value, Trap> {
    let val = eval(print.value);

    println!("{}", &val?);

    panic!("foo");
}

// TODO: move this parameter to something more generic
pub fn eval_str(_: Str) -> Result<Value, Trap> {
    panic!("eval_str: not impleted yet");
}
