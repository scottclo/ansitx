use std::{io, env};


#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor{
    fn move_left(&mut self, x: usize) {
        if x < self.x {
            self.x -= x;
        } else {
            self.x = 0;
        }
    }
    fn move_right(&mut self, x:usize) {
        self.x += x;
    }
    fn move_up(&mut self, y:usize) {
        if y < self.y {
            self.y -= y;
        } else {
            self.y = 0;
        }
    }
    fn move_down(&mut self, y:usize) {
        self.y += y;
    }
}


#[derive(Debug)]
struct Command {
    function: char,
    args: Vec<String>,
}

impl Command {
    fn new() -> Command{
        Command{
            function: char::default(),
            args: Vec::new(),
        }
    }
    fn get_arg_usize(&self, index: usize) -> usize {
        self.args[index].parse::<usize>().unwrap()
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let input: String = io::read_to_string(io::stdin()).unwrap();
    let mut ch_iter = input.chars();
    let mut is_ansi_escape: bool = false;
    let mut command: Command = Command::new();
    let mut cursor: Cursor = Cursor{x:0, y:0};
    let mut buffer: Vec<Vec<char>> = Vec::new();
    //print!("{}\n\n", input);
    while let Some(ch) = ch_iter.next() {
        if is_ansi_escape {
            if ch == '[' || ch == ';' {
                command.args.push(String::new());
            } else if ch == '?' || (ch >= '0' && ch <= '9') {
                let i = command.args.len() - 1;
                command.args[i].push(ch.clone());
            } else if ch.is_alphabetic() {
                command.function = ch.clone();
                is_ansi_escape = false;
                match command.function {
                    'A' => cursor.move_up(command.get_arg_usize(0)),
                    'B' => cursor.move_down(command.get_arg_usize(0)),
                    'C' => cursor.move_right(command.get_arg_usize(0)),
                    'D' => cursor.move_left(command.get_arg_usize(0)),
                    'm'|'l'|'h' => (),
                    _none => print!("{:?}{}: Not Implimented\n", command.args, command.function),
                }
            } else {
                is_ansi_escape = false;
            }
            continue;
        }
        if ch as u32 == 27 {
            is_ansi_escape = true;
            command = Command::new();
            continue;   
        } else if ch == '\n' {
            cursor.x = 0;
            cursor.y += 1;
            continue;
        } else {
            if buffer.len() <= cursor.y {
                buffer.resize_with(cursor.y + 1, Default::default);
            }
            buffer[cursor.y].resize(cursor.x + 1, ' ');
            buffer[cursor.y][cursor.x] = ch.clone();
            cursor.x += 1;
        }
    }
    for line in buffer{
        print!("{}\n", line.iter().collect::<String>());
    }
}
