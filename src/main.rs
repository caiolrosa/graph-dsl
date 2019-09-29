mod token;

fn main() {
    let mut tokenizer = token::Tokenizer::new();
    let input: &str = "
        graph {
            graph [bgcolor=\"yellow\"]
            a [color=\"red\"]
            b [color=\"blue\"]
            a -- b [color=\"green\"]
        }
    ";
    let tokens = tokenizer.parse(input);
    println!("{:?}", tokens);
}
