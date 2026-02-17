use std::io;
use std::collections::HashMap;

use parseagent::Guess;

fn main() -> io::Result<()> {
    let mut stats = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lines() {
        let e = stats.entry(Guess::new(&line?)).or_insert(0);
        *e += 1;
    }

    let mut stats: Vec<_> = stats.into_iter().collect();
    stats.sort_by_key(|(_k, v)| *v);
    stats.reverse();

    println!("Stats:");
    for (k, v) in stats {
        println!("{k}: {v}");
    }

    Ok(())
}
