mod lexer;

fn main() {
    if let Ok(tokens) = lexer::tokenize("1+2") {
        for token in tokens {
            println!("{:?}", token);
        }
    }
}
