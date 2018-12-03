extern crate advent_lib;
use advent_lib::read_line;

fn main() {
    let mut c: i64 = 0;
    while let Some(x) = read_line() {
        let n = match x[1..x.len()].trim().parse::<i64>() {
            Ok(i) => i,
            Err(_) => break,
        };
        c += if x.as_bytes()[0] == '+' as u8 {
            n
        } else {
            -n
        };
    }
    println!("{}", c);
}

