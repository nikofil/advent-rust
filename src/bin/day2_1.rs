extern crate advent_lib;
use advent_lib::read_line;
use std::collections::HashMap;

fn main() {
    let mut twos = 0;
    let mut threes = 0;
    while let Some(x) = read_line() {
        let mut m = HashMap::new();
        x.chars().for_each(|c| *m.entry(c).or_insert(0) += 1);
        if m.values().any(|v| *v == 2) {
            twos += 1;
        }
        if m.values().any(|v| *v == 3) {
            threes += 1;
        }
    }
    println!("{}", twos*threes);
}

