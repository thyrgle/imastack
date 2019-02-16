//! This module contains functions and structures related to the AST of the im-
//! astack language. Most notably, it converts between "string" *tokens* and e-
//! num representation.

pub mod float;

use std::str::FromStr;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(EnumString)]
pub enum Token {
    #[strum(serialize="+")]
    Add,
    #[strum(serialize="-")]
    Sub,
    #[strum(serialize="*")]
    Mul,
    #[strum(serialize="/")]
    Div,
    #[strum(serialize="dup")]
    Dup,
    #[strum(serialize="swp")]
    Swp,
    #[strum(serialize="jnz")]
    Jnz,
    #[strum(serialize="print")]
    Print,
    #[strum(default="true")]
    Number(float::Float)
}

impl Into<float::Float> for Token {
    /// Convets Token into Float.
    ///
    /// *Note* It tries the best it can, even though it doesn't make sense to 
    /// convert Token::Add to a float, it defaults this (as well as every other
    /// operator to Float(0.0).
    fn into(self) -> float::Float {
        match self {
            Token::Number(float::Float(x)) => float::Float(x),
            _                       => float::Float(0.0)
        }
    }
}

/// Compiles a vector of stringy tokens into the a vector of `Token`s.
/// 
/// *Note* It tires the best it can, if the token can't be parsed, convert it
/// to a `Float(0.0)` as default.
pub fn compile_program(tokens: &[&str]) -> Vec<Token> {
    let mut ast = Vec::new();
    for tok in tokens {
        let res = match Token::from_str(tok) {
            Ok(good_tok) => good_tok,
            _            => Token::Number(float::Float(0.0))
        };
        ast.push(res);
    }
    ast
}
