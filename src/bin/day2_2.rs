extern crate advent_lib;
use advent_lib::read_line;
use std::process::exit;

fn main() {
    let mut v = Vec::new();
    while let Some(x) = read_line() {
        v.push(x);
    }
    for (fi, first) in v.iter().enumerate() {
        for second in v[fi..].iter() {
            let (c, s) = first.chars().zip(second.chars()).fold((0, String::new()), |(acci, accs), (c1, c2)| {
                if c1 == c2 {
                    let mut accs2 = accs.clone();
                    accs2.push(c1);
                    (acci, accs2)
                } else {
                    (acci + 1, accs)
                }
            });
            if c == 1 {
                println!("{}", s);
                exit(0);
            }
        }
    }
}

