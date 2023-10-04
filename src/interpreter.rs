use crate::model::*;

pub fn eval(term: Box<Term>) -> Result<Value, Trap> {
    match *term {
        Term::Print(print) => eval_print(print),
        Term::Str(val) => Ok(Value::Str(val.value)),
        Term::Let(l) => eval_let(l),
    }
}

pub fn eval_print(print: Print) -> Result<Value, Trap> {
    let val = eval(print.value)?;
    println!("{}", val);
    Ok(val)
}

pub fn eval_let(l: Let) -> Result<Value, Trap> {
    panic!("not impleted yet");
}
