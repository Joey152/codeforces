// 50A
// solved: 6

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("error");
    
    let temp: Vec<_> = line.trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (m, n) = (temp[0], temp[1]);

    let width = m / 2;

    let extra = 
        if m % 2 == 0 {
            0
        }
        else {
            n / 2
        };

    let ans = width * n + extra;

    println!("{}", ans);
}
