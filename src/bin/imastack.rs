//! This the REPL for the imastack language. The output of the current line is
//! displayed at the bottom of the screen.
extern crate imastack;
extern crate ncurses;

use std::process;

/// Simple REPL for the imastack langauge.
fn main() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::keypad(ncurses::stdscr(), true);

    let mut code: Vec<String> = Vec::new();
    code.push(String::from(" "));
    let mut current_line: usize = 0;
    let mut current_col: usize = 0;
    loop {
        ncurses::mv(current_line as i32, current_col as i32);
        let ch = ncurses::getch();
        match ch {
            ncurses::KEY_ENTER => {
                code.push(String::from(" "));
                current_line += 1;
                current_col = 0;
            },
            // Also enter key...
            10 => {
                code.push(String::from(" "));
                current_line += 1;
                current_col = 0;
            },
            ncurses::KEY_UP => {
                match current_line {
                    0 => { },
                    _ => current_line -= 1,
                };
                current_col = 0;
            },
            ncurses::KEY_DOWN => {
                if current_line == code.len() {
                } else {
                    current_line += 1;
                }
                current_col = 0;
            },
            // Exit with Tab key.
            9 => process::exit(0),
            character => {
                code[current_line].push(character as u8 as char);
                ncurses::addch(character as u64);
                current_col += 1;
            }
        };
        let env = imastack::eval(&code[current_line].as_str());
        let mut footer = String::new();
        for num in env.output {
            footer.push_str(&num.to_string());
            footer.push(' ');
        }
        ncurses::mv(ncurses::LINES() - 1, 0);
        ncurses::clrtoeol();
        ncurses::mvprintw(ncurses::LINES() - 1,
                          ncurses::COLS() - footer.len() as i32,
                          &mut footer.to_string());
    }
}
