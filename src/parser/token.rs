#[derive(Debug)]
pub enum Token {
    LeftParen,  // (
    RightParen, // )
    LeftCurly,  // {
    RightCurly, // }
    Colon,      // :
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .

    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    Number(i32),
    Word(String),
}
