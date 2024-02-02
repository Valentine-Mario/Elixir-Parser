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
    Map(Option<(Box<Ast<'a>>, Verb)>, Vec<(&'a str, Ast<'a>)>),
    Array(Vec<Ast<'a>>),
    Tuple(Vec<Ast<'a>>),
    Boolean(bool),
    Variable { ident: String, expr: Box<Ast<'a>> },
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
    Merge,
}

fn flatten(nested: Vec<Vec<Ast>>) -> Vec<Ast> {
    nested.into_iter().flatten().collect()
}

fn parse_verb(pair: Pair<Rule>) -> Verb {
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
        "|" => Verb::Merge,
        _ => panic!("Unexpected  verb: {}", pair.as_str()),
    }
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

pub fn build_ast_from_stat(pair: Pair<Rule>) -> Vec<Ast> {
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

pub fn parse_types(pair: Pair<Rule>) -> Ast {
    match pair.as_rule() {
        Rule::number => Ast::Number(pair.as_str().parse().unwrap()),
        Rule::atom => Ast::Atom(pair.as_str()),
        Rule::string => Ast::String(pair.as_str()),
        Rule::single_quote_string => Ast::String(pair.as_str()),
        Rule::null => Ast::Null,
        Rule::boolean => Ast::Boolean(pair.as_str().parse().unwrap()),
        Rule::ident => Ast::Ident(pair.as_str()),
        Rule::array => Ast::Array(
            pair.into_inner()
                .map(|x| {
                    if x.as_rule()==Rule::ident{
                        return parse_types(x);
                    }else{
                        parse_types(x.into_inner().next().unwrap())
                    }
    })
                .collect(),
        ),
        Rule::tuple => Ast::Tuple(
            pair.into_inner()
                .map(|x| {
                    if x.as_rule()==Rule::ident{
                        return parse_types(x);
                    }else{
                        parse_types(x.into_inner().next().unwrap())
                    }
    })
                .collect(),
        ),
        Rule::arrow_first_map => {
            let mut merge_map: Option<(Box<Ast>, Verb)> = None;
            let mut map_items: Vec<(&str, Ast)> = vec![];
            let mut inner_rules = pair.into_inner();
            if inner_rules.len() > 3 {
                let mut a = inner_rules.clone();
                if a.next().unwrap().as_rule() == Rule::ident
                    && a.next().unwrap().as_rule() == Rule::verbs
                {
                    let ident = parse_types(inner_rules.next().unwrap());
                    let verb = parse_verb(inner_rules.next().unwrap());
                    merge_map = Some((Box::new(ident), verb))
                }
            }

            for i in inner_rules {
                match i.as_rule() {
                    Rule::arrow_map_item => {
                        let mut rule = i.into_inner();
                        let key = rule.next().unwrap().as_str();
                        let value: Ast;
                        if rule.clone().next().unwrap().as_rule() == Rule::ident{
                            value= parse_types(rule.next().unwrap())
                        }else{
                            value = parse_types(rule.next().unwrap().into_inner().next().unwrap());

                        }
                        map_items.push((key, value))
                    }
                    Rule::map_arrangment => {
                        let rule = i.into_inner();
                        for k in rule {
                            let key = k.clone().into_inner().next().unwrap().as_str();
                            let value: Ast;
                            if k.clone().into_inner().next_back().unwrap().as_rule() == Rule::ident{
                                value=parse_types(k.into_inner().next_back().unwrap());
                            }else{
                                value= parse_types(
                                    k.into_inner()
                                        .next_back()
                                        .unwrap()
                                        .into_inner()
                                        .next()
                                        .unwrap(),
                                );
                            }
                            
                            map_items.push((key, value))
                        }
                    }
                    Rule::colon_map_item => {
                        let mut rule = i.into_inner();
                        let key = rule.next().unwrap().as_str();
                        let value: Ast;

                        if rule.clone().next_back().unwrap().as_rule()==Rule::ident{
                            value= parse_types(rule.next_back().unwrap())
                        }else{
                        value =
                            parse_types(rule.next_back().unwrap().into_inner().next().unwrap());
                        }
                        
                        map_items.push((key, value))
                    }
                    _ => {}
                }
            }
            Ast::Map(merge_map, map_items)
        }
        Rule::colon_first_map => {
            let mut merge_map: Option<(Box<Ast>, Verb)> = None;
            let mut map_items: Vec<(&str, Ast)> = vec![];
            let mut inner_rules = pair.into_inner();
            if inner_rules.len() > 3 {
                let mut a = inner_rules.clone();
                if a.next().unwrap().as_rule() == Rule::ident
                    && a.next().unwrap().as_rule() == Rule::verbs
                {
                    let ident = parse_types(inner_rules.next().unwrap());
                    let verb = parse_verb(inner_rules.next().unwrap());
                    merge_map = Some((Box::new(ident), verb))
                }
            }

            for i in inner_rules {
                if i.as_rule() == Rule::colon_map_item {
                    let mut rule = i.into_inner();
                    let key = rule.next().unwrap().as_str();
                    let value: Ast;
                    if rule.clone().next_back().unwrap().as_rule()==Rule::ident{
                        value= parse_types(rule.next_back().unwrap())
                    }else{
                    value =
                        parse_types(rule.next_back().unwrap().into_inner().next().unwrap());
                    };
                    map_items.push((key, value))
                }
            }
            Ast::Map(merge_map, map_items)
        }
        _ => Ast::NoOp,
    }
}
