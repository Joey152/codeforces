// 282A
// solved: 8

use std::io::{self, Read};

fn main() {
    let mut lines = String::new();
    io::stdin()
        .read_to_string(&mut lines)
        .expect("error");

    let mut sum = 0;
    lines.lines()
        .skip(1)
        .for_each(|word| {
            match word.chars().find(|&x| x == '+') {
                Some(_) => sum += 1,
                None => sum -= 1,
            }
        });

    println!("{}", sum);
}
