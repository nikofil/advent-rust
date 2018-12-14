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
        let mut res: i128 = 0;
        let mut pres = 0;
        let mut df = 0;
        (0i128..200).for_each(|ii| {
            let mut cur = String::from("....");
            cur.push_str(&nxt);
            cur.push_str("....");
            nxt = String::new();
            (0..=cur.len()-5).for_each(|i| {
                nxt.push_str(m.get(&cur[i..i+5]).unwrap_or(&String::from(".")));
            });
            res = 0;
            nxt.chars().enumerate().for_each(|(i, x)| if x == '#' { res += i as i128-ii-ii-2 });
            df = res-pres;
            pres = res;
        });
        let mut fres: i128 = res as i128;
        fres += df * (50000000000-200);
        println!("{}", fres);
    }
}

