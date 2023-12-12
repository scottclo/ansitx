use std::io::{self, BufRead};

fn main() {
    let input: String = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut ch_iter = input.chars();
    while let Some(ch) = ch_iter.next() {
        if ch as u32 == 27 {
            let sequence: String = String::new();
            if (Some(ch) = ch_iter.next()) == '[' {

            }

        }
    }

}
