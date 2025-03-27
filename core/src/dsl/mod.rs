//! DSL Module
//! 

pub enum Expr {
    Literal(f64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    FunctionCall(String,Vec<Expr>),
}