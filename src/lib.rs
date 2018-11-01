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
    loop {
        let tok = tokens.get(reg);
        match tok {
            Some(&"+")     => words::add(stack),
            Some(&"-")     => words::sub(stack),
            Some(&"*")     => words::mul(stack),
            Some(&"/")     => words::div(stack),
            Some(&"dup")   => words::dup(stack),
            Some(&"swp")   => words::swp(stack),
            Some(&"jnz")   => words::jnz(stack, &mut reg),
            Some(&"print") => words::print_float(stack, output),
            Some(_)        => words::parse_number(tok.unwrap(), stack),
            None => break
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
pub fn eval(code: String) -> Vec<f64> {
    let tokens: Vec<&str> = code.split(' ').collect();
    let mut stack: Vec<f64> = Vec::new();
    let mut output: Vec<f64> = Vec::new();
    execute_program(tokens.as_slice(), &mut stack, &mut output)
}
