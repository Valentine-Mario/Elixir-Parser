use pest::iterators::Pair;

use crate::{
    ast::{parse_verb, Ast, Verb},
    Rule,
};

fn parse_sub_rule(token: Pair<Rule>) -> Ast {
    if token.as_rule() == Rule::ident {
        return parse_types(token);
    } else {
        let item = token.into_inner().next().unwrap();
        return parse_types(item);
    }
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
        Rule::array_op => {
            let mut inner = pair.into_inner();

            let first = inner.next().unwrap();
            let first_val = parse_sub_rule(first);

            let verb_item = inner.next().unwrap();
            let second_val = parse_verb(verb_item);

            let third = inner.next().unwrap();
            let third_val = parse_sub_rule(third);

            Ast::ArrayOp(Box::new(first_val), second_val, Box::new(third_val))
        }
        Rule::array => Ast::Array(pair.into_inner().map(|x| parse_sub_rule(x)).collect()),
        Rule::tuple => Ast::Tuple(pair.into_inner().map(|x| parse_sub_rule(x)).collect()),
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
                        let value: Ast = if rule.clone().next().unwrap().as_rule() == Rule::ident {
                            parse_types(rule.next().unwrap())
                        } else {
                            parse_types(rule.next().unwrap().into_inner().next().unwrap())
                        };

                        map_items.push((key, value))
                    }
                    Rule::map_arrangment => {
                        let rule = i.into_inner();
                        for k in rule {
                            let key = k.clone().into_inner().next().unwrap().as_str();
                            let value: Ast =
                                if k.clone().into_inner().next_back().unwrap().as_rule()
                                    == Rule::ident
                                {
                                    parse_types(k.into_inner().next_back().unwrap())
                                } else {
                                    parse_types(
                                        k.into_inner()
                                            .next_back()
                                            .unwrap()
                                            .into_inner()
                                            .next()
                                            .unwrap(),
                                    )
                                };

                            map_items.push((key, value))
                        }
                    }
                    Rule::colon_map_item => {
                        let mut rule = i.into_inner();
                        let key = rule.next().unwrap().as_str();
                        let value: Ast =
                            if rule.clone().next_back().unwrap().as_rule() == Rule::ident {
                                parse_types(rule.next_back().unwrap())
                            } else {
                                parse_types(rule.next_back().unwrap().into_inner().next().unwrap())
                            };

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
                    let value: Ast = if rule.clone().next_back().unwrap().as_rule() == Rule::ident {
                        parse_types(rule.next_back().unwrap())
                    } else {
                        parse_types(rule.next_back().unwrap().into_inner().next().unwrap())
                    };
                    map_items.push((key, value))
                }
            }
            Ast::Map(merge_map, map_items)
        }
        _ => Ast::NoOp,
    }
}
