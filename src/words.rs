//! The `word` module contains the verbs and nouns that create a program. Verbs
//! are functions (regardless of airity) and nouns are data.

pub struct Stack(pub Vec<f64>);

impl Stack {
    #[inline(always)]
    pub fn push(&mut self, item: f64) {
        self.0.push(item);
    }

    // helpful alias
    #[inline(always)]
    pub fn pop(&mut self) -> f64 {
        self.0.pop().unwrap_or(0.0)
    }

    /// Extracts two values off the top of a stack.
    ///
    /// # Arguments
    ///
    /// * `$stack` - stack to be mutated.
    #[inline(always)]
    pub fn get_ops(&mut self) -> (f64, f64) {
        (self.pop(), self.pop())
    }

    /// Parses a numerical value to a float.
    ///
    /// # Arguments
    ///
    /// `token` - The value to be converted to a float.
    /// `stack` - The stack to push the token onto.
    ///
    /// *Note* - If `parse_number` is **not** given a number, it will still return
    /// `0.0`.
    #[inline(always)]
    pub fn parse_number(&mut self, token: &str) {
        let number = token.parse::<f64>().unwrap_or(0.0);
        self.push(number);
    }

    /// Pops the top two elements off the stack and adds them.
    ///
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from and push onto.
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
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from and push onto.
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
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from and push onto.
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
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from and push onto.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used. If division by `0.0` occurs, then a value of `0.0` pushed
    /// to `stack` instead.
    #[inline(always)]
    pub fn div(&mut self) {
        let (a, b) = self.get_ops();
        if b == 0.0 {
            self.push(0.0);
        } else {
            self.push(a / b);
        }
    }

    /// Pops the top element off the stack and pushes two copies of it on the stack.
    ///
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from and push onto.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used, thus `0.0` is pushed on to the stack twice.
    #[inline(always)]
    pub fn dup(&mut self) {
        let to_dup = self.pop();
        self.push(to_dup);
        self.push(to_dup);
    }

    /// Pops the top two elements off the stack and swaps their values.
    ///
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from and push onto.
    ///
    /// *Note* - If no number is available to pop from the stack, a default value
    /// of `0.0` is used.
    #[inline(always)]
    pub fn swp(&mut self) {
        let (first, second) = self.get_ops();
        self.push(second);
        self.push(first);
    }

    /// Pops off two values off the stack. If the first value is not zero, take the
    /// value of the second value and jump to that location in code.
    ///
    /// # Arguments
    ///
    /// * `reg` - The the current location of the register.
    /// * `stack` - The stack to pop from and push onto.
    ///
    #[inline(always)]
    pub fn jnz(&mut self, reg: &mut usize) {
        let (cond, jump) = self.get_ops();
        if cond != 0.0 {
            *reg = jump as usize;
        }
    }

    /// Prints the top value of a particular stack.
    ///
    /// # Arguments
    ///
    /// * `stack` - The stack to pop from.
    /// * `output` - The output vector to push onto.
    ///
    /// *Note* - Does not "print" to stdout, instead it prints to the `output` par-
    /// ameter. This is for better debugging and test.
    #[inline(always)]
    pub fn print_float(&mut self, output: &mut Vec<f64>) {
        output.push(self.pop())
    }
}
