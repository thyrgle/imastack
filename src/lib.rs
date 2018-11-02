mod words;

/// Given a list of commands, execute the commands.
///
/// # Arguments
///
/// * `tokens` - A slice of tokens to be executed.
/// * `stack` - The stack to keep the current state of the program.
fn execute_program(tokens: &[&str], 
                   stack: &mut Vec<f64>,
                   output: &mut Vec<f64>) -> Vec<f64> {
    // Analogous to the role of a "register" for a Turing machine.
    let mut reg: usize = 0;
    while let Some(tok) = tokens.get(reg) {
        match tok {
            &"+"     => words::add(stack),
            &"-"     => words::sub(stack),
            &"*"     => words::mul(stack),
            &"/"     => words::div(stack),
            &"dup"   => words::dup(stack),
            &"swp"   => words::swp(stack),
            &"jnz"   => words::jnz(stack, &mut reg),
            &"print" => words::print_float(stack, output),
            _        => words::parse_number(tok, stack),
        }
        reg += 1;
    }
    output.to_vec()
}

/// Evaluates a string of code.
/// 
/// # Arguments
///
/// * `code` - The string of code to be executed.
///
/// *Note* The value returned is the "output" of the code. Output is not done
/// through stdout for easier debugging.
pub fn eval(code: &str) -> Vec<f64> {
    let tokens: Vec<&str> = code.split(' ').collect();
    let mut stack: Vec<f64> = Vec::new();
    let mut output: Vec<f64> = Vec::new();
    execute_program(tokens.as_slice(), &mut stack, &mut output)
}
