use std::{io, env};


#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor{
    fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }

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
    function: String,
    args: Vec<String>,
}

impl Command {
    fn new() -> Command{
        Command{
            function: String::new(),
            args: Vec::new(),
        }
    }
    fn get_arg_usize(&self, index: usize) -> Option<usize> {
        match self.args.get(index) {
            Some(s) => {
                match s.parse::<usize>() {
                    Ok(u) => Some(u),
                    Err(_) => None
                }
            },
            None => None
        }
    }
}

fn warning(quiet: &bool, message: String) {
    if quiet == &false {
        print!("Warning: {message}. Use \"ansitx -q\" do hide this message.\n");
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let args: Vec<String> = env::args().collect();
    let input: String = io::read_to_string(io::stdin()).unwrap();
    let mut ch_iter = input.chars();
    let mut mode: u32 = 0;
    let mut command: Command = Command::new();
    let mut cursor: Cursor = Cursor{x:0, y:0};
    let mut buffer: Vec<Vec<char>> = Vec::new();
    let mut quiet: bool = false;
    
    for args in &args[1..] {
        match args.as_str() {
            "-q" | "--quiet" => quiet = true,
            _ => ()
        }
    }

    while let Some(ch) = ch_iter.next() {
        match mode {
            0 => { //normal text
                if ch as u32 == 27 {
                    mode = 1;
                    command = Command::new();
                }else if ch == '\r' {
                    cursor.x = 0;
                }
                else if ch == '\n' {
                    cursor.x = 0;
                    cursor.y += 1;
                }else if ch as u32 > 0 && ch as u32 <= 31 {
                    warning(&quiet, format!("{:?} is not implemented", ch));
                }else {
                    if buffer.len() <= cursor.y {
                        buffer.resize_with(cursor.y + 1, Default::default);
                    }
                    buffer[cursor.y].resize(cursor.x + 1, ' ');
                    buffer[cursor.y][cursor.x] = ch.clone();
                    cursor.x += 1;
                }
            },
            1 => {//ansii escape
                if ch == ']' {
                    mode = 2;
                } else if ch == ')' {
                    mode = 3;
                }
                else if ch == '[' || ch == ';' {
                    command.args.push(String::new());
                } else if ch == '?' || (ch >= '0' && ch <= '9') {
                    let i = command.args.len() - 1;
                    command.args[i].push(ch.clone());
                } else if ch == '(' {
                    command.function.push(ch); 
                }else if ch.is_alphabetic() {
                    command.function.push(ch);
                    mode = 0;
                    match command.function.as_str() {
                        "A" => cursor.move_up(command.get_arg_usize(0).unwrap_or(1)),
                        "B" => cursor.move_down(command.get_arg_usize(0).unwrap_or(1)),
                        "d" => cursor.y = command.get_arg_usize(0).unwrap_or(0)-1,
                        "C" => cursor.move_right(command.get_arg_usize(0).unwrap_or(1)),
                        "D" => cursor.move_left(command.get_arg_usize(0).unwrap_or(1)),
                        "G" => cursor.x = command.get_arg_usize(0).unwrap_or(0)-1,
                        "H" => {
                            cursor.x = command.get_arg_usize(1).unwrap_or(1)-1;
                            cursor.y = command.get_arg_usize(0).unwrap_or(1)-1;
                        },
                        "J" => {
                            match command.get_arg_usize(0).unwrap_or(0) {
                                0 => {
                                    if buffer.len() > 0 {
                                        for i in 0..buffer.len()-1 {
                                            if i <= cursor.y {
                                                if buffer[i].len() > 0 {
                                                    for ii in 0..buffer[i].len()-1 {
                                                        if ii < cursor.x {
                                                            buffer[i][ii] = ' ';
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },

                                1 => {
                                    if buffer.len() > 0 {
                                        for i in buffer.len()-1..0 {
                                            if i > cursor.y {
                                                buffer.remove(i);
                                            } else if i == cursor.y {
                                                if buffer[i].len() > 0 {
                                                    for ii in buffer[i].len()-1..0 {
                                                        if ii < cursor.x {
                                                            buffer[i].remove(ii);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                2 | 3 => {
                                    cursor = Cursor::new();
                                    buffer = Vec::new();
                                }
                                t => warning(&quiet, format!("[{}J is not implemented", t)),
                            }
                        },
                        |"m"|"l"|"h"|"r"|"X"|"(B"=> (), // Ignored 
                        _none => warning(&quiet, format!("{:?}{}: Is not currently handled.", command.args, command.function)),
                    }
                } else {
                    mode = 0;
                }
            },
            2 => {//os command
                if ch as u32 == 7 {
                    mode = 0;
                }
            },
            3 => {//
                mode = 0;
            },
            u => print!("{u} is not a mode... How did we get here?")
        }
    }
    for line in buffer{
        print!("{}\n", line.iter().collect::<String>());
    }
}
