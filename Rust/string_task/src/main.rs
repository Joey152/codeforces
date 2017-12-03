// 118A
// solved: 5

use std::io;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let vowels = "aeiouy";
    let word = word.trim().to_lowercase();
    let word: Vec<_> = word.chars().filter(|&x| !vowels.contains(x)).collect();
    let mut new_word = String::new();
    for c in word {
        new_word.push('.');
        new_word.push(c);
    }
    println!("{}", new_word);
}
