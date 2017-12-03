// 71A
// solved: 3

use std::io::{self, Read};

fn main() {
    let mut lines = String::new();
    io::stdin().read_to_string(&mut lines).unwrap();
    lines.lines()
        .skip(1)
        .for_each(|word| {
            if word.len() <= 10 {
                println!("{}", word)
            }
            else {
                let mut char_word = word.chars();
                let shortened_word = format!(
                    "{}{}{}",
                    char_word.nth(0).unwrap(),
                    word.len() - 2,
                    char_word.nth(word.len()-2).unwrap());
                println!("{}", shortened_word);
            }
        });
}
