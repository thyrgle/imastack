mod words;

/// Given a list of commands, execute the commands.
///
/// # Arguments
///
/// * `tokens` - A slice of tokens to be executed.
/// * `stack` - The stack to keep the current state of the program.
fn execute_program(tokens: &[&str], 
                   stack: &mut words::Stack,
                   output: &mut Vec<f64>) -> Vec<f64> {
    // Analogous to the role of a "register" for a Turing machine.
    let mut reg: usize = 0;
    while let Some(tok) = tokens.get(reg) {
        match tok {
            &"+"     => stack.add(),
            &"-"     => stack.sub(),
            &"*"     => stack.mul(),
            &"/"     => stack.div(),
            &"dup"   => stack.dup(),
            &"swp"   => stack.swp(),
            &"jnz"   => stack.jnz(&mut reg),
            &"print" => stack.print_float(output),
            _        => stack.parse_number(tok),
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
    let mut stack = words::Stack {0: Vec::new()};
    let mut output: Vec<f64> = Vec::new();
    execute_program(tokens.as_slice(), &mut stack, &mut output)
}
