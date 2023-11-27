use elixir_parser::ElixirParser;
use pest::Parser;

fn main() {
    let unparsed_file = std::fs::read_to_string("app.ex").expect("cannot read elixir file");
    let ast = ElixirParser::parse(elixir_parser::Rule::program, &unparsed_file);
    println!("{:?}", ast);
}
