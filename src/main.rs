mod lexer;
use lexer::Token;

fn main() {
    if let Ok(tokens) = lexer::tokenize("1+2") {
        for token in tokens {
            println!("{:?}", token);
        }
    }
}

fn evaluate(text: &str) -> f64 {
    if let Ok(tokens) = lexer::tokenize(text) {
        for token in tokens {
            match token {
                _ => ()
            }
        }
    }

    0.0
}
