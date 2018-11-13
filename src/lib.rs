extern crate strum;
#[macro_use]
extern crate strum_macros;

mod ast;
mod words;

use ast::Token;
use ast::Float;
use words::Stack;
use std::str::FromStr;

/// Given a list of commands, execute the commands.
///
/// # Arguments
///
/// * `tokens` - A slice of tokens to be executed.
/// * `stack` - The stack to keep the current state of the program.
fn execute_program(tokens: &[Token], 
                   stack: &mut Stack,
                   output: &mut Vec<Float>) -> Vec<Float> {
    // Analogous to the role of a "register" for a Turing machine.
    let mut reg: usize = 0;
    while let Some(tok) = tokens.get(reg) {
        match tok {
            Token::Add     => stack.add(),
            Token::Sub     => stack.sub(),
            Token::Mul     => stack.mul(),
            Token::Div     => stack.div(),
            Token::Dup     => stack.dup(),
            Token::Swp     => stack.swp(),
            Token::Jnz     => stack.jnz(&mut reg),
            Token::Print   => stack.print_float(output),
            _              => stack.push_number(&mut (*tok).into()),
        }
        reg += 1;
    }
    output.to_vec()
}

pub fn compile_program<'a>(tokens: &[&str]) -> &'a [Token] {
    let mut ast: Vec<Token> = Vec::new();
    for token in tokens {
        ast.push(Token::from_str(token).unwrap());
    }
    ast.as_slice()
}

/// Evaluates a string of code.
/// 
/// # Arguments
///
/// * `code` - The string of code to be executed.
///
/// *Note* The value returned is the "output" of the code. Output is not done
/// through stdout for easier debugging.
pub fn eval(code: &str) -> Vec<Float> {
    let tokens: Vec<&str> = code.split(' ').collect();
    let mut stack = Stack {0: Vec::new()};
    let mut output: Vec<Float> = Vec::new();
    let ast = compile_program(tokens.as_slice());
    execute_program(ast, &mut stack, &mut output)
}
