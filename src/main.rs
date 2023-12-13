use std::io::{self, BufRead};

fn main() {
    let input: String = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut ch_iter = input.chars();
    let mut is_ansi_escape: bool = false;
    let mut command: String = String::new();
    while let Some(ch) = ch_iter.next() {
        
        if is_ansi_escape {
            if ch == '[' || ch == '?' || ch == ';' || ch == '{' || ch == '}' || (ch >= '0' && ch <= '9') {
                command.push(ch.clone());
            } else if ch.is_alphabetic() {
                command.push(ch.clone());
                is_ansi_escape = false;
                match command.chars().last().unwrap() {
                    'm' => (),
                    'h' => print!("\n\nHome: {}\n", command),
                    'f' => print!("\n\nMoveTo: {}\n", command),
                    none => print!("Command: {}, ", none),
                }
            } else {
                print!("\nCommand: {}\n", command);
                print!("'{}' is not a valid character for ansi escape sequence.", ch);
                is_ansi_escape = false;
            }
        }

        if ch as u32 == 27 {
            is_ansi_escape = true;
            command = String::new();
        }
    }
}
