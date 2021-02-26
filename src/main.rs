extern crate nom;
use nom::{
  IResult,
  branch::{alt},
  character::complete::{alphanumeric1,digit1,one_of},
  combinator::{map_res},
  sequence::{tuple},
};

#[derive(Debug,PartialEq)]
pub enum BinOp {
  Add,
  Sub,
  Mul,
  Div,
  Exp,
  Log
}

#[derive(Debug,PartialEq)]
pub enum Expr {
  Int64(i64),
  Iden(String),
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

fn value(input: &str) -> IResult<&str, Expr> {
  let (input, v) = alt((int64, identifier))(input)?;

  Ok((input, v))
}

fn binop(input: &str) -> IResult<&str, Expr> {
  let (input, (e1, op, e2)) = tuple((value, one_of("+-*/"), value))(input)?;
  Ok((input, Expr::BinOp(
        match op {
          '+' => BinOp::Add,
          '-' => BinOp::Sub,
          '*' => BinOp::Mul,
          '/' => BinOp::Div,
          _ => unreachable!()
        }, Box::new(e1), Box::new(e2))))
}

fn main() {}

#[test]
fn parse_id() {
  assert_eq!(identifier("foo"), Ok(("", Expr::Iden("foo".to_string()))));
}

#[test]
fn parse_i64() {
  assert_eq!(int64("42"), Ok(("", Expr::Int64(42))));
}

#[test]
fn parse_val() {
  assert_eq!(value("foo"), Ok(("", Expr::Iden("foo".to_string()))));
  assert_eq!(value("42"), Ok(("", Expr::Int64(42))));
}

#[test]
fn parse_binop() {
  let res = binop("foo+42");
  match res {
    Ok(("", Expr::BinOp(op, e1, e2))) => {
      assert_eq!(op, BinOp::Add);
      assert_eq!(*e1, Expr::Iden("foo".to_string()));
      assert_eq!(*e2, Expr::Int64(42));
    }
    _ => assert_eq!(true, false)
  };
}
