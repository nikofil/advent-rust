extern crate advent_lib;
extern crate regex;
use advent_lib::read_line;
use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    if let Some(x) = read_line() {
        let s = x[15..].trim().to_string();
        read_line();
        while let Some(x) = read_line() {
            let s = x.split(' ').collect::<Vec<&str>>();
            m.insert(s[0].to_string(), s[2].to_string());
        }
        let mut nxt = s;
        (0..20).for_each(|_| {
            let mut cur = String::from("....");
            cur.push_str(&nxt);
            cur.push_str("....");
            nxt = String::new();
            (0..=cur.len()-5).for_each(|i| {
                nxt.push_str(m.get(&cur[i..i+5]).unwrap_or(&String::from(".")));
            });
            println!("{}", nxt);
        });
        let mut res: i64 = 0;
        nxt.chars().enumerate().for_each(|(i, x)| if x == '#' { res += i as i64-40 });
        println!("{}", res);
    }
}

