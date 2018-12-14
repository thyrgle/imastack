//! This module contains functions and structures related to the AST of the im-
//! astack language. Most notably, it converts between "string" *tokens* and e-
//! num representation.

use std::str::FromStr;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;


/// Wrapper for `f64`. 
///
/// *Note* This is needed for `strum` to operate correctly as `From<&'a str>`
/// needs to be implemented, which is impossible with a bare `f64`.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Float(pub f64);

impl<'a> From<&'a str> for Float {
    #[inline]
    fn from(s: &'a str) -> Self {
        Float(s.parse::<f64>().unwrap_or(0.0).to_owned())
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<usize> for Float {
    #[inline]
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Add for Float {
    type Output = Float;
    
    #[inline]
    fn add(self, other: Float) -> Float {
        Float(self.0 + other.0)
    }
}


impl Sub for Float {
    type Output = Float;

    #[inline]
    fn sub(self, other: Float) -> Float {
        Float(self.0 - other.0)
    }
}

impl Mul for Float {
    type Output = Float;
    
    #[inline]
    fn mul(self, other: Float) -> Float {
        Float(self.0 * other.0)
    }
}

impl Div for Float {
    type Output = Float;
    
    #[inline]
    fn div(self, other: Float) -> Float {
        Float(self.0 / other.0)
    }
}


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
    Number(Float)
}

impl Into<Float> for Token {
    /// Convets Token into Float.
    ///
    /// *Note* It tries the best it can, even though it doesn't make sense to 
    /// convert Token::Add to a float, it defaults this (as well as every other
    /// operator to Float(0.0).
    fn into(self) -> Float {
        match self {
            Token::Number(Float(x)) => Float(x),
            _                       => Float(0.0)
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
            _            => Token::Number(Float(0.0))
        };
        ast.push(res);
    }
    ast
}
