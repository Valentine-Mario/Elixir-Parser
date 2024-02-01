use elixir_parser::{parse, ElixirParser};
use pest::Parser;

fn main() {
    let unparsed_file = std::fs::read_to_string("app.ex").expect("cannot read elixir file");
    let ast = parse(&unparsed_file);
    println!("{:?}", ast);
    // let ast = ElixirParser::parse(elixir_parser::Rule::program, &unparsed_file);
    // println!("{:?}", ast);
}
