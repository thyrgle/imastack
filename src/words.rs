//! The `word` module contains the verbs and nouns that create a program. Verbs
//! are functions (regardless of airity) and nouns are data.

/// Extracts two values off the top of a stack.
///
/// # Arguments
///
/// * `$stack` - stack to be mutated.
macro_rules! get_ops {
    ($stack:expr) => {
        ($stack.pop().unwrap_or(0.0),
        $stack.pop().unwrap_or(0.0))
    }
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
pub fn parse_number(token: &str, stack: &mut Vec<f64>) {
    let number = token.parse::<f64>().unwrap_or(0.0);
    stack.push(number);
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
pub fn add(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    stack.push(a + b);
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
pub fn sub(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    stack.push(a - b);
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
pub fn mul(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    stack.push(a * b);
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
pub fn div(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    if b == 0.0 {
        stack.push(0.0);
    } else {
        stack.push(a / b);
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
pub fn dup(stack: &mut Vec<f64>) {
    let to_dup = stack.pop().unwrap_or(0.0);
    stack.push(to_dup);
    stack.push(to_dup);
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
pub fn swp(stack: &mut Vec<f64>) {
    let (first, second) = get_ops!(stack);
    stack.push(second);
    stack.push(first);
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
pub fn jnz(stack: &mut Vec<f64>, reg: &mut usize) {
    let (cond, jump) = get_ops!(stack);
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
pub fn print_float(stack: &mut Vec<f64>, output: &mut Vec<f64>) {
    output.push(stack.pop().unwrap_or(0.0))
}
