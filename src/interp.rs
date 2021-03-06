use std::io::Write;
use std::collections::HashMap;

use crate::Err;
use crate::ast::{BinOp, Cmd, Expr, Single};
use crate::parse::expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Whole(u64),
    Integer(i64),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub struct Env {
    bindings: HashMap<String, Value>
}

impl Env {
    pub fn new() -> Env {
        Env { bindings: HashMap::new() }
    }
}

fn eval_iden(env: &Env, i: &str) -> Result<Value, Err> {
    match env.bindings.get(i) {
        Some(v) => Ok(v.clone()),
        _ => {
            println!("no binding for {}: {:?}", i, env);
            Err(Err::Env)
        }
    }
}

fn eval_single(env: &mut Env, e: Single) -> Result<Value, Err> {
    match e {
        Single::Whole(u) => Ok(Value::Whole(u)),
        Single::Integer(i) => Ok(Value::Integer(i)),
        Single::Str(s) => Ok(Value::Str(s)),
        Single::Iden(i) => eval_iden(env, &i),
        Single::Paren(e) => eval(env, *e)
    }
}

fn eval_cmd(env: &mut Env, cmd: Cmd) -> Result<Value, Err> {
    let mut proc = std::process::Command::new(cmd.cmd);
    for arg in cmd.args {
        let _ = match eval(env, arg)? {
            Value::Whole(u) => proc.arg(u.to_string()),
            Value::Integer(i) => proc.arg(i.to_string()),
            Value::Str(a) => proc.arg(a),
        };
    }
    let output = proc.output().expect("failed to execute cmd");
    let s = String::from_utf8(output.stdout).expect("invalid UTF-8 output");
    Ok(Value::Str(s.to_string()))
}

fn eval_binop(env: &mut Env, op: BinOp, left: Box<Expr>, right: Box<Expr>) -> Result<Value, Err> {
    let lv = eval(env, *left)?;
    let rv = eval(env, *right)?;
    match (lv, rv) {
        (Value::Whole(l), Value::Whole(r)) =>
            match op {
                BinOp::Add => Ok(Value::Whole(l+r)),
                BinOp::Sub => Ok(Value::Whole(l-r)),
                BinOp::Mul => Ok(Value::Whole(l*r)),
                BinOp::Div => Ok(Value::Whole(l/r)),
            },
        _ => Err(Err::Eval)
    }
}

fn eval_cond(env: &mut Env, pred: Box<Expr>, br1: Box<Expr>, br2: Box<Expr>) -> Result<Value, Err> {
    let p = eval(env, *pred)?;
    let truth =
        match p {
            Value::Whole(u) => u > 0,
            Value::Integer(i) => i != 0,
            Value::Str(s) => !s.is_empty()
        };
    if truth {
        eval(env, *br1)
    } else {
        eval(env, *br2)
    }
}

fn eval_assign(env: &mut Env, name: &str, expr: Box<Expr>) -> Result<Value, Err> {
    let v = eval(env, *expr)?;
    env.bindings.insert(name.to_string(), v.clone());
    Ok(v.clone())
}

fn eval(env: &mut Env, expr: Expr) -> Result<Value, Err> {
    match expr {
        Expr::Single(e) => eval_single(env, e),
        Expr::Cmd(cmd) => eval_cmd(env, cmd),
        Expr::BinOp(op, e1, e2) => eval_binop(env, op, e1, e2),
        Expr::Cond(pred, br1, br2) => eval_cond(env, pred, br1, br2),
        Expr::Assign(n, expr) => eval_assign(env, &n, expr),
    }
}

fn parse(input: &str) -> Result<Expr, Err> {
    match expr(input) {
        Ok(("", e)) => Ok(e),
        _ => Err(Err::Parse)
    }
}

fn parse_cmd(input: &str) -> Result<std::process::Command, Err> {
    match crate::parse::cmd::cmd(input) {
        Ok(("", c)) => Ok(c),
        _ => Err(Err::Parse)
    }
}

fn run_cmd(env: &mut Env, mut cmd: std::process::Command) -> Result<String, Err> {
    let output = cmd.output().expect("failed to execute cmd");
    Ok(String::from_utf8(output.stdout).expect("invalid UTF-8 output"))
}

fn read() -> Result<String, Err> {
    let mut l = String::new();
    match std::io::stdin().read_line(&mut l) {
        Ok(_) => Ok(l),
        _ => Err(Err::Read)
    }
}

fn read_eval(env: &mut Env) -> Result<String, Err> {
    let line = read()?;
    /*
    let expr = parse(&line)?;
    match eval(env, expr)? {
    match eval(env, expr)? {
        // TODO: Display trait
        Value::Whole(u) => Ok(format!("{}: u64", u)),
        Value::Integer(i) => Ok(format!("{}: i64", i)),
        Value::Str(s) => Ok(format!("{}: str", s))
    }
    */
    let mut cmd = parse_cmd(&line)?;
    run_cmd(env, cmd)
}

pub fn repl() {
    let mut env = Env::new();
    loop {
        print!("$ ");
        std::io::stdout().flush().expect("failed to flush prompt");
        match read_eval(&mut env) {
            Ok(s) => println!("{}", s),
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
