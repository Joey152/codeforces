// 1A
// solved: 2

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let temp: Vec<_> = line.trim().split(" ").map(|x| x.parse::<f64>().unwrap()).collect();
    let (a, b, c) = (temp[0], temp[1], temp[2]);
    let width = (a / c).ceil();
    let length = (b / c).ceil();
    let total = width * length;
    println!("{}", total);
}
