use crate::syntax::{Bop, Exp, Type, Uop};
use crate::{Keyword, Pos, PosToken, Token};

// ML : Exp                                                    { $1 }
//
// -- rules --
//
// Type : '()'                                                 { TUnit }
//      | 'int'                                                { TInt }
//      | 'bool'                                               { TBool }
//      | Type '->' Type                                       { TArrow $1 $3 }
//      | Type '*' Type                                        { TProd $1 $3 }
//      | Type '+' Type                                        { TSum $1 $3 }
//      | 'ref' Type                                           { TRef $2 }
//      | '(' Type ')'                                         { $2 }
//
//
// Exp : 'fun' '(' id ':' Type ')' '->' Exp                    { Abs (posn $1) (name $3) $5 Nothing $8 }
//     | 'if' Exp 'then' Exp 'else' Exp                        { ITE (posn $1) $2 $4 $6 }
//     | 'let' id ':' Type '=' Exp 'in' Exp                    { Let (posn $1) (name $2) $4 $6 $8 }
//     | 'let' 'rec' id '(' id ':' Type ')' ':' Type '=' Exp 'in' Exp
//                                                             { LetRec (posn $1) (name $3) (name $5) $7 $10 $12 $14 }
//     | 'case' Exp 'of' '|' 'inl' id '->' Exp '|' 'inr' id '->' Exp
//                                                             { Case (posn $1) $2 (name $6) $8 (name $11) $13 }
//     | ExpBin                                                { $1 }

struct Parser {
    tokens: Vec<PosToken>,
    index: usize,
}

impl Parser {
    /// Return the next token that has not yet been processed, or None if
    /// reached end-of-file, and mark it as processed. OK to call repeatedly
    /// after reaching EOF.
    fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.index).map(|token| token.token.clone());
        self.index += 1;
        token
    }

    fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.index).map(|token| token.token.clone())
    }

    /// Return the byte position within the query string at which the
    /// next token starts.
    fn pos(&self) -> usize {
        match self.tokens.get(self.index) {
            Some(token) => token.pos,
            None => usize::MAX,
        }
    }

    fn consume_token(&mut self, expected: &Token) -> bool {
        match &self.peek_token() {
            Some(t) if *t == *expected => {
                self.next_token();
                true
            }
            _ => false,
        }
    }

    /// Bail out if the current token is not an expected token, or consume it if it is
    fn expect_token(&mut self, expected: &Token) -> Result<(), anyhow::Error> {
        if self.consume_token(expected) {
            Ok(())
        } else {
            panic!()
        }
    }

    fn maybe_parse<T, F>(&mut self, mut f: F) -> Option<T>
    where
        F: FnMut(&mut Self) -> Result<T, anyhow::Error>,
    {
        let index = self.index;
        if let Ok(t) = f(self) {
            Some(t)
        } else {
            self.index = index;
            None
        }
    }

    fn parse_exp(&mut self) -> Result<Exp, anyhow::Error> {
        todo!();
    }

    fn parse_type(&mut self) -> Result<Type, anyhow::Error> {
        todo!();
    }

    /// Parse a new expression
    fn parse_expr(&mut self) -> Result<Exp, anyhow::Error> {
        self.parse_subexpr(Precedence::Zero)
    }

    /// Parse tokens until the precedence decreases
    fn parse_subexpr(&mut self, precedence: Precedence) -> Result<Exp, anyhow::Error> {
        let expr = self.parse_prefix()?;
        self.parse_subexpr_seeded(precedence, expr)
    }

    fn parse_subexpr_seeded(
        &mut self,
        precedence: Precedence,
        mut expr: Exp,
    ) -> Result<Exp, anyhow::Error> {
        loop {
            let next_precedence = self.get_next_precedence();
            if precedence >= next_precedence {
                break;
            }

            expr = self.parse_infix(expr, next_precedence)?;
        }
        Ok(expr)
    }

    // Parse an expression prefix
    fn parse_prefix(&mut self) -> Result<Exp, anyhow::Error> {
        if self.consume_token(&Token::Not) {
            let pos = self.pos();
            let expr = self.parse_prefix()?;
            Ok(Exp::Uop(pos, Uop::Not, Box::new(expr)))
        } else {
            self.parse_exp_app()
        }
    }

    /// Parse an operator following an expression
    fn parse_infix(&mut self, expr: Exp, precedence: Precedence) -> Result<Exp, anyhow::Error> {
        Ok(match self.next_token() {
            Some(Token::Assign) => Exp::Asgn(
                self.pos(),
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Or) => Exp::Bop(
                self.pos(),
                Bop::Or,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::And) => Exp::Bop(
                self.pos(),
                Bop::Or,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Eqeq) => Exp::Bop(
                self.pos(),
                Bop::Eq,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Neq) => Exp::Bop(
                self.pos(),
                Bop::Neq,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Ge) => Exp::Bop(
                self.pos(),
                Bop::Ge,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Le) => Exp::Bop(
                self.pos(),
                Bop::Le,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Gt) => Exp::Bop(
                self.pos(),
                Bop::Gt,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Lt) => Exp::Bop(
                self.pos(),
                Bop::Lt,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Plus) => Exp::Bop(
                self.pos(),
                Bop::Plus,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Minus) => Exp::Bop(
                self.pos(),
                Bop::Minus,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Mult) => Exp::Bop(
                self.pos(),
                Bop::Mul,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Div) => Exp::Bop(
                self.pos(),
                Bop::Div,
                Box::new(expr),
                Box::new(self.parse_subexpr(precedence)?),
            ),
            Some(Token::Keyword(Keyword::Ref)) => Exp::Ref(self.pos(), Box::new(expr)),
            Some(Token::Bang) => Exp::Deref(self.pos(), Box::new(expr)),
            _ => panic!(),
        })
    }

    /// Get the precedence of the next token
    fn get_next_precedence(&self) -> Precedence {
        match self.peek_token() {
            Some(Token::Assign) => Precedence::Assign,
            Some(Token::Arrow) => Precedence::Arrow,
            Some(Token::Or) => Precedence::Or,
            Some(Token::And) => Precedence::And,
            Some(Token::Eqeq | Token::Neq | Token::Ge | Token::Le | Token::Gt | Token::Lt) => {
                Precedence::Cmp
            }
            Some(Token::Plus | Token::Minus) => Precedence::PlusMinus,
            Some(Token::Mult | Token::Div) => Precedence::MultiplyDivide,
            Some(Token::Keyword(Keyword::Ref) | Token::Bang) => Precedence::RefDeref,
            None => Precedence::Zero,
            _ => panic!(),
        }
    }

    fn parse_exp_app(&mut self) -> Result<Exp, anyhow::Error> {
        // There must be at least one
        let mut exprs = vec![self.parse_exp_un2()?];
        // Try to parse as many as possible
        while let Some(expr) = self.maybe_parse(Self::parse_exp_un2) {
            exprs.push(expr);
        }
        // Construct the right associative AST nodes
        todo!()
    }

    fn parse_exp_un2(&mut self) -> Result<Exp, anyhow::Error> {
        Ok(match self.next_token() {
            Some(Token::Keyword(Keyword::Ref)) => {
                let inner = self.parse_exp_un2()?;
                Exp::Deref(self.pos(), Box::new(inner))
            }
            Some(Token::Keyword(Keyword::Fst)) => {
                let inner = self.parse_exp_un2()?;
                Exp::Fst(self.pos(), Box::new(inner))
            }
            Some(Token::Keyword(Keyword::Snd)) => {
                let inner = self.parse_exp_un2()?;
                Exp::Snd(self.pos(), Box::new(inner))
            }
            Some(Token::Keyword(Keyword::Inl)) => {
                self.expect_token(&Token::LParen)?;
                let typ = self.parse_type()?;
                self.expect_token(&Token::RParen)?;
                let inner = self.parse_exp_un2()?;
                Exp::Inl(self.pos(), typ, Box::new(inner))
            }
            Some(Token::Keyword(Keyword::Inr)) => {
                self.expect_token(&Token::LParen)?;
                let typ = self.parse_type()?;
                self.expect_token(&Token::RParen)?;
                let inner = self.parse_exp_un2()?;
                Exp::Inr(self.pos(), typ, Box::new(inner))
            }
            // ExpAtom
            Some(Token::Keyword(Keyword::True)) => Exp::BoolLit(self.pos(), true),
            Some(Token::Keyword(Keyword::False)) => Exp::BoolLit(self.pos(), true),
            Some(Token::Unit) => Exp::Unit(self.pos()),
            Some(Token::Ident(name)) => Exp::Var(self.pos(), name),
            Some(Token::LParen) => {
                let fst = self.parse_exp()?;
                match self.next_token() {
                    Some(Token::Comma) => {
                        let snd = self.parse_exp()?;
                        self.expect_token(&Token::RParen)?;
                        Exp::Pair(self.pos(), Box::new(fst), Box::new(snd))
                    }
                    Some(Token::RParen) => fst,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        })
    }
}

/// Defines a number of precedence classes operators follow. Since this enum derives Ord, the
/// precedence classes are ordered from weakest binding at the top to tightest binding at the
/// bottom.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Precedence {
    Zero,
    Assign,
    Arrow,
    Or,
    And,
    Cmp,
    PlusMinus,
    MultiplyDivide,
    RefDeref,
}
