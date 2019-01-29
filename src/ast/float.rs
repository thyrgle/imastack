//! Thin wrapper of primitive f64 type. Needed for use with `strum` library.

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
