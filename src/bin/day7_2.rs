extern crate advent_lib;
use advent_lib::read_line;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let mut req: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut fin = HashMap::new();
    let mut busy = vec![0u32; 5];
    while let Some(s) = read_line() {
        let ln = s.split(' ' as char).collect::<Vec<&str>>();
        req.entry(ln[7].to_string().chars().next().unwrap() as u8).or_insert(Vec::new()).push(ln[1].to_string().chars().next().unwrap() as u8);
    }
    while fin.len() < 26 {
        let l = ('A' as u8..='Z' as u8).filter(
            |l| req.get(l).unwrap_or(&Vec::new()).iter().all(
                |x| fin.contains_key(x)) && !fin.contains_key(l)).max().unwrap();
        let cur = req.get(&l).unwrap_or(&Vec::new()).iter().map(|x| fin[x]).max().unwrap_or(0);
        let (i, _) = busy.iter().enumerate().min_by(|(_i1, x1), (_i2, x2)| x1.cmp(x2)).unwrap();
        let cur = max(cur, busy[i]);
        busy[i] = cur + l as u32 - 'A' as u32 + 61;
        fin.insert(l as u8, busy[i]);
        println!("{} {}", l as char, busy[i]);
    }
    println!("{}", busy.iter().max().unwrap());
}
