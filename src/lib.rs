use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::Parser;

mod ast;
mod types;
use ast::Ast;
use types::parse_types;

#[derive(Parser)]
#[grammar = "grammer/elixir.pest"]
pub struct ElixirParser;

fn flatten(nested: Vec<Vec<Ast>>) -> Vec<Ast> {
    nested.into_iter().flatten().collect()
}

pub fn parse(source: &str) -> Result<Vec<Ast>, Box<Error<Rule>>> {
    let mut ast_return = vec![];
    let pairs = ElixirParser::parse(Rule::program, source)?;
    for pair in pairs {
        if pair.as_rule() == Rule::stat {
            ast_return.push(build_ast_from_stat(pair));
        }
    }

    Ok(flatten(ast_return))
}

fn build_ast_from_stat(pair: Pair<Rule>) -> Vec<Ast> {
    let mut ast_return = vec![];
    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::types => ast_return.push(parse_types(item.into_inner().next().unwrap())),
            Rule::expr => {
                println!("expression {:?}", item.as_str())
            }
            Rule::ident => {
                ast_return.push(parse_types(item));
            }
            _ => {}
        }
    }
    ast_return
}
