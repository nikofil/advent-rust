extern crate advent_lib;
use advent_lib::read_line;
use std::collections::HashSet;

fn main() {
    let mut c: i64 = 0;
    let mut s = HashSet::new();
    let mut v = Vec::new();
    while let Some(x) = read_line() {
        let n = match x[1..x.len()].trim().parse::<i64>() {
            Ok(i) => i,
            Err(_) => break,
        };
        v.push(if x.as_bytes()[0] == '+' as u8 {
            n
        } else {
            -n
        });
    }
    'outer: loop {
        for x in &v {
            c += x;
            if !s.insert(c) {
                break 'outer;
            }
        }
    }
    println!("{}", c);
}

