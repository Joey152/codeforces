// 231A
// solved: 7

use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    
    let stdin = io::stdin();
    let mut io = stdin.lock();

    io.read_line(&mut line).expect("error");
    let n: i32 = line.trim().parse().unwrap();

    let mut sum = 0;
    for _ in 0..n {
        let mut line = String::new();
        io.read_line(&mut line).expect("error");
        let line: Vec<i32> = line
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let (a, b, c) = (line[0], line[1], line[2]);
        let solved = a + b + c;
        
        if solved >= 2 {
            sum += 1;
        }
    }

    println!("{}", sum);
}
