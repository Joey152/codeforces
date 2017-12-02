// 4A
// solved: 1

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line: i32 = line.trim().parse().unwrap();
    if line <= 2 || line % 2 != 0 {
        println!("NO");
    }
    else {
        println!("YES");
    }
}