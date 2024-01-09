use std::ffi::CString;

use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/elixir.pest"]
pub struct ElixirParser;


#[derive(Clone, Debug)]
enum Ast<'a>{
    Number(f64),
    Ident(String),
    String(CString),
    Null,
    Map(Vec<(&'a str, Ast<'a>)>),
    Array(Vec<Ast<'a>>),
    Tuple(Vec<Ast<'a>>),
    Boolean(bool),
}