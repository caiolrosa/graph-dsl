mod token;
mod parser;
mod graph;

fn main() {
    let mut tokenizer = token::Tokenizer::new();
    let mut parser = parser::Parser::new();
    let input: &str = "
        graph {
            graph =\"yellow\"]
            a [color=\"red\"]
            b [color=\"blue\"]
            a -- b [color=\"green\"]
        }
    ";
    let tokens = tokenizer.parse(input);
    let graph = parser.parse(tokens);
    println!("{:?}", graph);
}
