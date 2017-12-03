// 158A
// solved: 4

use std::io;

fn main() {
    let mut nk = String::new();
    let mut scores = String::new();
    io::stdin().read_line(&mut nk).unwrap();
    io::stdin().read_line(&mut scores).unwrap();
    let nk: Vec<_> = nk.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut scores: Vec<_> = scores.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let n = nk[0];
    let k = nk[1];
    scores.sort();
    let score_to_beat = scores[n-k];
    let result = scores.into_iter().filter(|&x| x >= score_to_beat && x > 0).collect::<Vec<_>>().len();
    println!("{}", result);
}
