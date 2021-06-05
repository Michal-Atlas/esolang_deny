use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
pub(crate) enum Token {
    #[regex(" ?= ?")]
    Assign,

    #[token("<<")]
    Intake,

    #[token("+")]
    Sum,

    #[token("-")]
    Diff,

    #[token("*")]
    Product,

    #[token("/")]
    Div,

    #[token("%")]
    Mod,

    #[token(">>")]
    Print,
    #[token("#>")]
    Printf,

    #[regex("[a-zA-Z]+")]
    Variable,

    #[regex("[0-9]+")]
    Number,

    #[token("\n")]
    NewLine,
    #[error]
    #[regex(r"[ \t\n\f ]+", logos::skip)]
    Other,
}
