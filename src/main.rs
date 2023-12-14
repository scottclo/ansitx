use std::io;

struct Vec2 {
    x: usize,
    y: usize,
}

fn main() {
    let input: String = io::read_to_string(io::stdin()).unwrap();
    let mut ch_iter = input.chars();
    let mut is_ansi_escape: bool = false;
    let mut command: String = String::new();
    let mut cursor: Vec2 = Vec2{x:0, y:0};
    let mut buffer: Vec<Vec<char>> = Vec::new();
    print!("{}\n\n\n\n", input);
    print!("{:?}\n\n\n\n", input);

    while let Some(ch) = ch_iter.next() {
        if is_ansi_escape {
            if ch == '[' || ch == '?' || ch == ';' || ch == '{' || ch == '}' || (ch >= '0' && ch <= '9') {
                command.push(ch.clone());
                continue;
            } else if ch.is_alphabetic() {
                command.push(ch.clone());
                is_ansi_escape = false;
                match command.chars().last().unwrap() {
                    _none => (),
                }
                continue;
            } else {
                is_ansi_escape = false;
                continue;
            }
        }

        if ch as u32 == 27 {
            is_ansi_escape = true;
            command = String::new();
            continue;   
        } else if ch == '\n' {
            cursor.x = 0;
            cursor.y += 1;
            continue;
        } else {
            buffer.resize_with(cursor.y + 1, Default::default);
            buffer[cursor.y].resize(cursor.x + 1, ' ');
            buffer[cursor.y][cursor.x] = ch.clone();
            cursor.x += 1;
        }
    }

    for line in buffer{
        print!("{:?}\n", line);
    }
}
