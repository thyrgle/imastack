use std::str::FromStr;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;


pub struct Float(pub f64);

impl Clone for Float {
    fn clone(&self) -> Float { Float(self.0) }
}

impl<'a> From<&'a str> for Float {
    #[inline]
    fn from(s: &'a str) -> Self {
        Float(s.parse::<f64>().unwrap_or(0.0).to_owned())
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

pub fn compile_program<'a>(tokens: &[&str]) -> &'a [Token] {
    let mut ast: Vec<Token> = Vec::new();
    for token in tokens {
        ast.push(Token::from_str(token).unwrap());
    }
    ast.as_slice()
}

