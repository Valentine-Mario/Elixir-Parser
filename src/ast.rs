use crate::Rule;
use pest::iterators::Pair;

#[derive(Clone, Debug)]
pub enum Ast<'a> {
    Number(f64),
    Ident(&'a str),
    String(&'a str),
    Atom(&'a str),
    Null,
    NoOp,
    Map(Option<(Box<Ast<'a>>, Verb)>, Vec<(&'a str, Ast<'a>)>),
    Array(Vec<Ast<'a>>),
    ArrayOp(Box<Ast<'a>>, Verb, Box<Ast<'a>>),
    Tuple(Vec<Ast<'a>>),
    Boolean(bool),
    Assignment {
        ident: Box<Ast<'a>>,
        expr: Box<Ast<'a>>,
    },
}

#[derive(Clone, Debug)]
pub enum Verb {
    Plus,
    Times,
    LessThan,
    LargerThan,
    Equal,
    Minus,
    Divide,
    Concat,
    ConcatString,
    And,
    Or,
    Pipe,
    Stroke,
}

pub fn parse_verb(pair: Pair<Rule>) -> Verb {
    match pair.as_str() {
        "+" => Verb::Plus,
        "*" => Verb::Times,
        "-" => Verb::Minus,
        "<" => Verb::LessThan,
        "=" => Verb::Equal,
        ">" => Verb::LargerThan,
        "%" => Verb::Divide,
        "++" => Verb::Concat,
        "<>" => Verb::ConcatString,
        "&&" => Verb::And,
        "||" => Verb::Or,
        "|" => Verb::Stroke,
        "|>" => Verb::Pipe,
        _ => panic!("Unexpected  verb: {}", pair.as_str()),
    }
}
