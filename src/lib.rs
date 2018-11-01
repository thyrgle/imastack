/// Extracts two values off the top of a stack. 
///
/// # Arguments
macro_rules! get_ops {
    ($stack:expr) => {
        ($stack.pop().unwrap_or(0.0),
         $stack.pop().unwrap_or(0.0))
    }
}

fn parse_number(token: &str, stack: &mut Vec<f64>) {
    let number = token.parse::<f64>().unwrap_or(0.0);
    stack.push(number);
}

/// Pops the top two elements off the stack and adds them.
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn add(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    stack.push(a + b);
}

/// Pops the top two elements off the stack and subtracts them.
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn sub(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    stack.push(a - b);
}

/// Pops the top two elements off the stack and multiplies them.
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn mul(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    stack.push(a * b);
}

/// Pops the top two elements off the stack and divides them.
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn div(stack: &mut Vec<f64>) {
    let (a, b) = get_ops!(stack);
    if b == 0.0 {
        stack.push(0.0);
    } else {
        stack.push(a / b);
    }
}

/// Pops the top element off the stack and pushes two copies of it on the stack.
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn dup(stack: &mut Vec<f64>) {
    let to_dup = stack.pop().unwrap_or(0.0);
    stack.push(to_dup);
    stack.push(to_dup);
}

/// Pops the top two elements off the stack and swaps their values.
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn swp(stack: &mut Vec<f64>) {
    let (first, second) = get_ops!(stack);
    stack.push(first);
    stack.push(second);
}

/// Pops the top two elements off the stack and subtracts them. TODO
/// # Arguments
///
/// * `stack` - The stack to be mutated.
fn jnz(i: &mut usize, stack: &mut Vec<f64>) {
    let (cond, jump) = get_ops!(stack);
    if cond != 0.0 {
        *i = jump as usize;
    }
}

/// Prints the top value of a particular stack.
///
/// # Arguments
/// * `stack` - The stack to be mutated.
fn print_float(stack: &mut Vec<f64>) {
    println!("{}", stack.pop().unwrap_or(0.0));
}

/// Given a list of commands, execute the commands.
///
/// # Arguments
///
/// * `tokens` - A slice of tokens to be executed.
/// * `stack` - The stack to keep the current state of the program.
fn execute_program(tokens: &[&str], stack: &mut Vec<f64>) {
    let mut i: usize = 0;
    loop {
        match tokens.get(i) {
            Some(tok) => {
                match *tok {
                    "+"     => add(stack),
                    "-"     => sub(stack),
                    "*"     => mul(stack),
                    "/"     => div(stack),
                    "dup"   => dup(stack),
                    "swp"   => swp(stack),
                    "jnz"   => jnz(&mut i, stack),
                    "print" => print_float(stack),
                    _       => parse_number(tok, stack)
                }
            },
            None => break
        }
        i += 1;
    }
}

/// Evaluates a string of code.
///
/// * `code` - The string of code to be executed.
pub fn eval(code: String) {
    let tokens: Vec<&str> = code.split(' ').collect();
    let mut stack: Vec<f64> = Vec::new();
    execute_program(tokens.as_slice(), &mut stack);
}
