use crate::model::*;

pub fn eval(term: Box<Term>) -> Result<Value, Trap> {
    match *term {
        Term::Print(print) => eval_print(print),
        Term::Str(val) => Ok(Value::Str(val.value)),
        Term::Let(l) => eval_let(l),
        Term::Function(func) => eval_function(func),
        Term::If(i) => eval_if(i),
        Term::Binary(b) => eval_binary(b),
    }
}

pub fn eval_print(print: Print) -> Result<Value, Trap> {
    let val = eval(print.value)?;
    println!("{}", val);
    Ok(val)
}

pub fn eval_let(l: Let) -> Result<Value, Trap> {
    panic!("eval_let: not implemented yet");
}

pub fn eval_function(func: Function) -> Result<Value, Trap> {
    panic!("eval_function: not implemented yet");
}

pub fn eval_if(i: If) -> Result<Value, Trap> {
    panic!("eval_if: not implemented yet");
}

pub fn eval_binary(b: Binary) -> Result<Value, Trap> {
    panic!("eval_binary: not implemented yet");
}
