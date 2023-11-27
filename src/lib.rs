use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/elixir.pest"]
pub struct ElixirParser;
