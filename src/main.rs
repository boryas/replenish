extern crate nom;
use nom::{
  IResult,
  branch::{alt},
  character::complete::{alphanumeric1,digit1,multispace0,multispace1,one_of},
  combinator::{map_res},
  multi::{separated_list1},
  sequence::{tuple},
};

#[derive(Debug,PartialEq)]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug,PartialEq)]
pub enum Expr {
  Int64(i64),
  Iden(String),
  Apply(Vec<Expr>),
  BinOp(BinOp, Box<Expr>, Box<Expr>)
}

fn identifier(input: &str) -> IResult<&str, Expr> {
  let (input, id) = alphanumeric1(input)?;

  Ok((input, Expr::Iden(id.to_string())))
}

fn int64(input: &str) -> IResult<&str, Expr> {
  let (input, i) = map_res(
    digit1,
    |ds: &str| i64::from_str_radix(&ds, 10))(input)?;

  Ok((input, Expr::Int64(i)))
}

fn binop(input: &str) -> IResult<&str, Expr> {
  let (input, (e1, _, op, _, e2)) = tuple((expression, multispace0, one_of("+-*/"), multispace0, expression))(input)?;
  Ok((input, Expr::BinOp(
        match op {
          '+' => BinOp::Add,
          '-' => BinOp::Sub,
          '*' => BinOp::Mul,
          '/' => BinOp::Div,
          _ => unreachable!()
        }, Box::new(e1), Box::new(e2))))
}

fn apply(input: &str) -> IResult<&str, Expr> {
  // todo: parse it as "fn args"
  let (input, v) = separated_list1(multispace1, expression)(input)?;
  Ok((input, Expr::Apply(v)))
}

fn expression(input: &str) -> IResult<&str, Expr> {
  let (input, e) = alt((int64, identifier, apply, binop))(input)?;
  Ok((input, e))
}

fn main() {
  println!("{:?}", apply("foo 42 43 44"));
}

#[test]
fn parse_id() {
  assert_eq!(identifier("foo"), Ok(("", Expr::Iden("foo".to_string()))));
}

#[test]
fn parse_i64() {
  assert_eq!(int64("42"), Ok(("", Expr::Int64(42))));
}

#[test]
fn parse_simple_expr() {
  assert_eq!(expression("foo"), Ok(("", Expr::Iden("foo".to_string()))));
  assert_eq!(expression("42"), Ok(("", Expr::Int64(42))));
}

#[test]
fn parse_binop() {
  let res = binop("foo + 42");
  match res {
    Ok(("", Expr::BinOp(op, e1, e2))) => {
      assert_eq!(op, BinOp::Add);
      assert_eq!(*e1, Expr::Iden("foo".to_string()));
      assert_eq!(*e2, Expr::Int64(42));
    }
    _ => assert_eq!(true, false)
  };
}

#[test]
fn parse_apply() {
  let res = apply("foo 42");
  match res {
    Ok(("", Expr::Apply(e1, e2))) => {
      assert_eq!(*e1, Expr::Iden("foo".to_string()));
      assert_eq!(*e2, Expr::Int64(42));
    }
    _ => assert_eq!(true, false)
  };
}
