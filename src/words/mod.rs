//! The `word` module contains the verbs and nouns that create a program. Verbs
//! are functions (regardless of airity) and nouns are data.

use ast::Float;

pub struct Env {
    pub stack: Vec<Float>,
    pub output: Vec<Float>
}

impl Env {

    #[inline(always)]
    pub fn push(&mut self, item: Float) {
        self.stack.push(item);
    }

    // helpful alias
    #[inline(always)]
    pub fn pop(&mut self) -> Float {
        self.stack.pop().unwrap_or(Float(0.0))
    }

    /// Extracts two values off the top of a stack.
    #[inline(always)]
    pub fn get_ops(&mut self) -> (Float, Float) {
        (self.pop(), self.pop())
    }

    /// Parses a numerical value to a float.
    ///
    /// # Arguments
    ///
    /// `token` - The value to be converted to a float.
    ///
    /// *Note* - If `parse_number` is **not** given a number, it will still return
    /// `0.0`.
    #[inline(always)]
    pub fn push_number(&mut self, number: Float) {
        self.push(number);
    }

    /// Pops the top two elements off the stack and adds them.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value 
    /// of `0.0` is used.
    #[inline(always)]
    pub fn add(&mut self) {
        let (a, b) = self.get_ops();
        self.push(a + b);
    }

    /// Pops the top two elements off the stack and subtracts them.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used.
    #[inline(always)]
    pub fn sub(&mut self) {
        let (a, b) = self.get_ops();
        self.push(a - b);
    }

    /// Pops the top two elements off the stack and multiplies them.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used.
    #[inline(always)]
    pub fn mul(&mut self) {
        let (a, b) = self.get_ops();
        self.push(a * b);
    }

    /// Pops the top two elements off the stack and divides them.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used. If division by `0.0` occurs, then a value of `0.0` pushed
    /// to `stack` instead.
    #[inline(always)]
    pub fn div(&mut self) {
        let (a, b) = self.get_ops();
        if b.0 == 0.0 {
            self.push(Float(0.0));
        } else {
            self.push(a / b);
        }
    }

    /// Pops the top element off the stack and pushes two copies of it on the stack.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used, thus `0.0` is pushed on to the stack twice.
    #[inline(always)]
    pub fn dup(&mut self) {
        let to_dup = self.pop();
        let copy = to_dup.clone();
        self.push(to_dup);
        self.push(copy);
    }

    /// Pops the top two elements off the stack and swaps their values.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used.
    #[inline(always)]
    pub fn swp(&mut self) {
        let (first, second) = self.get_ops();
        self.push(first);
        self.push(second);
    }

    /// Pops off two values off the stack. If the first value is not zero, take the
    /// value of the second value and jump to that location in code.
    ///
    /// # Arguments
    ///
    /// * `reg` - The the current location of the register.
    ///
    #[inline(always)]
    pub fn jnz(&mut self, reg: &mut usize) {
        let (cond, jump) = self.get_ops();
        if cond.0 != 0.0 {
            *reg = jump.into();
        }
    }

    /// Prints the top value of a particular stack.
    ///
    /// # Arguments
    ///
    /// * `output` - The output vector to push onto.
    ///
    /// *Note* - Does not "print" to stdout, instead it prints to the `output` par-
    /// ameter. This is for better debugging and test.
    #[inline(always)]
    pub fn print_float(&mut self) {
        let popped = self.pop();
        self.output.push(popped);
    }
}
