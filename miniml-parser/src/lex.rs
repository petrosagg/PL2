use anyhow::bail;

use crate::{Keyword, PosToken, Token};

struct LexBuf<'a> {
    buf: &'a str,
    pos: usize,
}

impl<'a> LexBuf<'a> {
    fn new(buf: &'a str) -> Self {
        LexBuf { buf, pos: 0 }
    }
    pub fn peek(&self) -> Option<char> {
        self.buf[self.pos..].chars().next()
    }
    /// Advances the internal cursor past the next character in the buffer if
    /// the character is `ch`.
    ///
    /// Returns whether the cursor advanced.
    pub fn consume(&mut self, ch: char) -> bool {
        if self.peek() == Some(ch) {
            self.next();
            true
        } else {
            false
        }
    }
    pub fn prev(&mut self) -> char {
        if let Some(c) = self.buf[..self.pos].chars().rev().next() {
            self.pos -= c.len_utf8();
            c
        } else {
            panic!("LexBuf::prev called on buffer at position 0")
        }
    }
}

impl<'a> Iterator for LexBuf<'a> {
    type Item = char;

    /// Returns the next character in the buffer, if any, advancing the internal
    /// cursor past the character.
    ///
    /// It is safe to call `next` after it returns `None`.
    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(c) = c {
            self.pos += c.len_utf8();
        }
        c
    }
}

pub fn lex(query: &str) -> Result<Vec<PosToken>, anyhow::Error> {
    let buf = &mut LexBuf::new(query);
    let mut tokens = vec![];
    while let Some(ch) = buf.next() {
        let pos = buf.pos - ch.len_utf8();
        let token = match ch {
            _ if ch.is_ascii_whitespace() => continue,
            '!' => Token::Bang,
            '&' if buf.consume('&') => Token::And,
            '(' if buf.consume(')') => Token::Unit,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '*' => Token::Mult,
            '+' => Token::Plus,
            ',' => Token::Comma,
            '/' if buf.consume('=') => Token::Neq,
            '/' => Token::Div,
            ':' if buf.consume('=') => Token::Assign,
            ':' => Token::Colon,
            '<' if buf.consume('=') => Token::Le,
            '<' => Token::Lt,
            '=' if buf.consume('=') => Token::Eqeq,
            '=' => Token::Eq,
            '>' if buf.consume('=') => Token::Ge,
            '>' => Token::Gt,
            '|' if buf.consume('|') => Token::Or,
            '|' => Token::Bar,
            '~' => Token::Not,
            '-' if buf.consume('>') => Token::Arrow,
            '-' if matches!(buf.peek(), Some('0'..='9')) => lex_literal(buf),
            '-' => Token::Minus,
            'A'..='Z' | 'a'..='z' | '_' => lex_ident(buf),
            '0'..='9' => lex_literal(buf),
            _ => bail!("unexpected character in input: {}", ch),
        };
        tokens.push(PosToken { token, pos })
    }

    Ok(tokens)
}

fn lex_ident(buf: &mut LexBuf) -> Token {
    buf.prev();
    let pos: usize = buf.pos;
    let ident: String = buf
        .take_while(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_'))
        .collect();
    match &*ident {
        "let" => Token::Keyword(Keyword::Let),
        "rec" => Token::Keyword(Keyword::Rec),
        "in" => Token::Keyword(Keyword::In),
        "fun" => Token::Keyword(Keyword::Fun),
        "ref" => Token::Keyword(Keyword::Ref),
        "if" => Token::Keyword(Keyword::If),
        "then" => Token::Keyword(Keyword::Then),
        "else" => Token::Keyword(Keyword::Else),
        "fst" => Token::Keyword(Keyword::Fst),
        "snd" => Token::Keyword(Keyword::Snd),
        "true" => Token::Keyword(Keyword::True),
        "false" => Token::Keyword(Keyword::False),
        "case" => Token::Keyword(Keyword::Case),
        "of" => Token::Keyword(Keyword::Of),
        "inl" => Token::Keyword(Keyword::Inl),
        "inr" => Token::Keyword(Keyword::Inr),
        "bool" => Token::Keyword(Keyword::Bool),
        "int" => Token::Keyword(Keyword::Int),
        _ => Token::Ident(ident),
    }
}

fn lex_literal(buf: &mut LexBuf) -> Token {
    let sign: i64 = if buf.consume('-') { -1 } else { 1 };

    let mut n: String = buf.take_while(|ch| matches!(ch, '0'..='9')).collect();
    let n: i64 = n.parse().unwrap();
    Token::Literal(sign * n)
}
