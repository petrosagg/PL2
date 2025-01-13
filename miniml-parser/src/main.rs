mod lex;
mod parse;
mod syntax;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Keyword {
    Let,
    Rec,
    In,
    Fun,
    Ref,
    If,
    Then,
    Else,
    Fst,
    Snd,
    True,
    False,
    Case,
    Of,
    Inl,
    Inr,
    Int,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Keyword(Keyword),
    Ident(String),
    Literal(i64),
    Assign,
    Arrow,
    Bang,
    Plus,
    Minus,
    Mult,
    Div,
    Eqeq,
    Neq,
    Ge,
    Le,
    Gt,
    Lt,
    Not,
    And,
    Or,
    LParen,
    RParen,
    Unit,
    Eq,
    Comma,
    Colon,
    Bar,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PosToken {
    token: Token,
    pos: usize,
}

type Pos = usize;

fn main() {}
