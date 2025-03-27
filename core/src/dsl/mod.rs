//! DSL Module
//! 

// use nom::{
//     branch::alt,
//     bytes::complete::tag,
//     character::complete::{alpha1,digit1,multispace0,one_of},
//     multi::separated_list0,
//     sequence::{delimited,preceded,pair,Tuple},
//     IResult,
//     combinator::{map, opt},
// };

/// DSL Expression Parser
pub enum Expr {
    /// Numerical literal
    Literal(f64),
    /// Variable
    Variable(String),
    /// Sum
    Add(Box<Expr>, Box<Expr>),
    /// Difference
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication
    Mul(Box<Expr>, Box<Expr>),
    /// Division
    Div(Box<Expr>, Box<Expr>),
    /// Function
    FunctionCall(String,Vec<Expr>),
}