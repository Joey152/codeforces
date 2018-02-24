// 96A
// solved: 9

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("error");

    let mut counter = 1;
    let mut players = line.trim().chars();
    let mut prev = players.nth(0).unwrap();
    for c in players {
        
        if counter == 7 {
            break;
        }

        if prev == c {
            counter += 1;
        }
        else {
            counter = 1;
        }
        prev = c;
    }

    if counter == 7 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
