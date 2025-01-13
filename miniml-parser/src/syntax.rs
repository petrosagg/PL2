use crate::Pos;

pub struct Type;

pub enum Exp {
    Uop(Pos, Uop, Box<Exp>),
    Bop(Pos, Bop, Box<Exp>, Box<Exp>),
    App(Pos, Box<Exp>, Box<Exp>),
    Fst(Pos, Box<Exp>),
    Snd(Pos, Box<Exp>),
    Inl(Pos, Type, Box<Exp>),
    Inr(Pos, Type, Box<Exp>),
    BoolLit(Pos, bool),
    NumLit(Pos, i64),
    Unit(Pos),
    Var(Pos, String),
    Pair(Pos, Box<Exp>, Box<Exp>),

    Deref(Pos, Box<Exp>),
    Ref(Pos, Box<Exp>),
    Asgn(Pos, Box<Exp>, Box<Exp>),
}

pub enum Uop {
    Not,
}

pub enum Bop {
    Plus,
    Minus,
    Mul,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Neq,
}
