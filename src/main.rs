mod parser;

use parser::Lexer;

fn main() {
    let s = String::from(
        r"function add(a: number, b: number): number {
    return a + b;
}
console.log(add(1, 2));
",
    );
    let mut lexer = Lexer::new(s);
    let tokens = lexer.lex();
    dbg!(&tokens);
    for token in tokens.unwrap() {
        println!("{:?}", token);
    }
}
