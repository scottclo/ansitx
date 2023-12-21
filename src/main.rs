use std::{io, env};

struct ScreenBuffer{
    state : Vec<Vec<char>>,
}

impl ScreenBuffer{
    fn new() -> ScreenBuffer {
        ScreenBuffer{ state : Vec::new() }
    }
    fn clear(&mut self) {
        self.state = Vec::new();
    }
    fn print(&mut self) {
        for line in self.state.iter(){
            print!("{}\n", line.iter().collect::<String>());
        }
    }
    fn clear_line_at(&mut self, cursor: &ScreenCursor){
        self.state[cursor.y] = Vec::new();
        self.expand_to_cursor(cursor);
    }
    fn clear_line_before(&mut self, cursor: &ScreenCursor){
        for i in 0..cursor.x {
            self.state[cursor.y][i] = ' ';
        }
    }
    fn clear_line_after(&mut self, cursor: &ScreenCursor){
        for i in (cursor.x..self.state[cursor.y].len()).rev(){
            self.state[cursor.y].remove(i);
        }
    }
    fn clear_screen_before(&mut self, cursor: &ScreenCursor) {
        for i in 0..=cursor.y {
            if i == cursor.y {
                for ii in 0..=cursor.x {
                    self.state[i][ii] = ' ';
                }
            } else {
                self.state[i].clear();
            }
        }
    }
    fn clear_screen_after(&mut self, cursor: &ScreenCursor) {
        for i in (cursor.y..self.state.len()).rev() {
            if i == cursor.y {
                for ii in (cursor.x..self.state[i].len()).rev() {
                    self.state[i].remove(ii);
                }
            } else {
                self.state.remove(i);
            }
        }
    }
    fn expand_to_cursor(&mut self, cursor: &ScreenCursor) {
        if self.state.len() <= cursor.y {
            self.state.resize_with(cursor.y + 1, Default::default);
        }
        if self.state[cursor.y].len() <= cursor.x {
            self.state[cursor.y].resize(cursor.x + 1, ' ');
        }
    }
    fn set_at_cursor(&mut self, cursor: &ScreenCursor, ch: char){
        self.expand_to_cursor(cursor);
        self.state[cursor.y][cursor.x] = ch;
    }
}

#[derive(Debug)]
struct ScreenCursor {
    x: usize,
    y: usize,
}

impl ScreenCursor{
    fn new() -> ScreenCursor {
        ScreenCursor { x: 0, y: 0 }
    }
    fn reset(&mut self) {
        (self.x, self.y) = (0,0)
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
    fn set_x(&mut self, x:usize) {
        self.x = x;
    }
    fn set_y(&mut self, y:usize) {
        self.y = y;
    }
    fn next_line(&mut self, n:usize) {
        self.move_down(n);
        self.set_x(0);
    }
    fn previous_line(&mut self, n:usize) {
        self.move_up(n);
        self.set_x(0);
    }
    fn horazontal_tab(&mut self) {
        self.x = self.x + self.x % 8;
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
    let args: Vec<String> = env::args().collect();
    let input: String = io::read_to_string(io::stdin()).unwrap();
    let mut ch_iter = input.chars();
    let mut mode: u32 = 0;
    let mut command: Command = Command::new();
    let mut cursor: ScreenCursor = ScreenCursor::new();
    let mut buffer: ScreenBuffer = ScreenBuffer::new();
    let mut quiet: bool = false;

    for args in &args[1..] {
        match args.as_str() {
            "-d" | "--debug" => env::set_var("RUST_BACKTRACE", "full"),
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
                    cursor.set_x(0);
                }else if ch as u32 == 8 {
                    cursor.move_left(1);
                }else if ch == '\n' || ch as u32 == 11 {
                    cursor.next_line(1)
                }else if ch == '\t' {
                    cursor.horazontal_tab();
                }
                else if ch as u32 > 0 && ch as u32 <= 31 {
                    warning(&quiet, format!("ASCII Control: {:?} is not implemented", ch));
                }else {
                    buffer.set_at_cursor(&cursor, ch.clone());
                    cursor.move_right(1);
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
                        "C" => cursor.move_right(command.get_arg_usize(0).unwrap_or(1)),
                        "D" => cursor.move_left(command.get_arg_usize(0).unwrap_or(1)),
                        "d" => cursor.set_y(command.get_arg_usize(0).unwrap_or(0)-1),
                        "E" => cursor.next_line(command.get_arg_usize(0).unwrap_or(1)),
                        "F" => cursor.previous_line(command.get_arg_usize(0).unwrap_or(1)),
                        "G" => cursor.set_x(command.get_arg_usize(0).unwrap_or(0)-1),
                        "H"|"f" => {
                            cursor.set_x(command.get_arg_usize(1).unwrap_or(1)-1);
                            cursor.set_y(command.get_arg_usize(0).unwrap_or(1)-1);
                        },
                        "J" => { //erase in display
                            match command.get_arg_usize(0).unwrap_or(0) {
                                0 => buffer.clear_screen_after(&cursor),
                                1 => buffer.clear_screen_before(&cursor),
                                2 | 3 => {
                                    cursor.reset();
                                    buffer.clear();
                                },
                                t => warning(&quiet, format!("[{}J is not implemented", t)),
                            }
                        },
                        "K" => { //erase in line
                            match command.get_arg_usize(0).unwrap_or(0) {
                                0 => buffer.clear_line_after(&cursor),
                                1 => buffer.clear_line_before(&cursor),
                                2 => buffer.clear_line_at(&cursor),
                                t => warning(&quiet, format!("[{}K is not implemented", t)),
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
            3 => {//some nonsense
                mode = 0;
            },
            u => warning(&quiet, format!("{u} is not a mode... How did we get here?"))
        }
    }
    buffer.print();
}
