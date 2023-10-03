use crate::model::{self, Term};

pub fn eval(term: Box<model::Term>) {
    match *term {
        Term::Print(print) => eval_print(print),
    }
}

// TODO: move this parameter to something more generic
pub fn eval_print(print: model::Print) {
    panic!("not impleted yet");
}
