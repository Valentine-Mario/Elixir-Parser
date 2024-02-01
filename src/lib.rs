use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer/elixir.pest"]
pub struct ElixirParser;

#[derive(Clone, Debug)]
pub enum Ast<'a> {
    Number(f64),
    Ident(&'a str),
    String(&'a str),
    Atom(&'a str),
    Null,
    NoOp,
    Map(Vec<(&'a str, Ast<'a>)>),
    Array(Vec<Ast<'a>>),
    Tuple(Vec<Ast<'a>>),
    Boolean(bool),
    Variable { ident: String, expr: Box<Ast<'a>> },
}

fn flatten(nested: Vec<Vec<Ast>>) -> Vec<Ast> {
    nested.into_iter().flatten().collect()
}

pub fn parse(source: &str) -> Result<Vec<Ast>, Error<Rule>> {
    let mut ast_return = vec![];
    let pairs = ElixirParser::parse(Rule::program, source)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::stat => {
                ast_return.push(build_ast_from_stat(pair));
            }
            _ => {}
        }
    }
    Ok(flatten(ast_return))
}

pub fn build_ast_from_stat(pair: Pair<Rule>) -> Vec<Ast> {
    let mut ast_return = vec![];
    for item in pair.clone().into_inner() {
        match item.as_rule() {
            Rule::types => ast_return.push(parse_types(item.into_inner().next().unwrap())),
            _ => {}
        }
    }
    ast_return
}

pub fn parse_types(pair: Pair<Rule>) -> Ast {
    match pair.as_rule() {
        Rule::number => Ast::Number(pair.as_str().parse().unwrap()),
        Rule::atom => Ast::Atom(pair.as_str()),
        Rule::string => Ast::String(pair.as_str()),
        Rule::single_quote_string => Ast::String(pair.as_str()),
        Rule::null => Ast::Null,
        Rule::boolean => Ast::Boolean(pair.as_str().parse().unwrap()),
        Rule::array => Ast::Array(
            pair.into_inner()
                .map(|x| parse_types(x.into_inner().next().unwrap()))
                .collect(),
        ),
        _ => Ast::NoOp,
    }
}
