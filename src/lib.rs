extern crate strum;
#[macro_use]
extern crate strum_macros;

pub mod ast;
pub mod words;

use ast::Token;
use words::Env;

/// Given a list of commands, execute the commands.
///
/// # Arguments
///
/// * `tokens` - A slice of tokens to be executed.
/// * `stack` - The stack to keep the current state of the program.
fn execute_program(tokens: &[Token], 
                   env: &mut Env) {
    // Analogous to the role of a "register" for a Turing machine.
    let mut reg: usize = 0;
    while let Some(tok) = tokens.get(reg) {
        match tok {
            Token::Add       => env.add(),
            Token::Sub       => env.sub(),
            Token::Mul       => env.mul(),
            Token::Div       => env.div(),
            Token::Dup       => env.dup(),
            Token::Swp       => env.swp(),
            Token::Jnz       => env.jnz(&mut reg),
            Token::Print     => env.print_float(),
            Token::Number(x) => env.push_number(x.clone()),
        }
        reg += 1;
    }
}

/// Evaluates a string of code.
/// 
/// # Arguments
///
/// * `code` - The string of code to be executed.
///
/// *Note* The value returned is the "output" of the code. Output is not done
/// through stdout for easier debugging.
pub fn eval(code: &str) -> Env {
    let tokens: Vec<&str> = code.split(' ').collect();
    let mut env = Env {
        stack: Vec::new(),
        output: Vec::new(),
    };
    let ast = ast::compile_program(tokens.as_slice());
    execute_program(ast.as_slice(), &mut env);
    env
}
