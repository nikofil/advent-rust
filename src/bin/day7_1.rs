extern crate advent_lib;
use advent_lib::read_line;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut req: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut done = HashSet::new();
    while let Some(s) = read_line() {
        let ln = s.split(' ' as char).collect::<Vec<&str>>();
        req.entry(ln[7].to_string().chars().next().unwrap() as u8).or_insert(Vec::new()).push(ln[1].to_string().chars().next().unwrap() as u8);
    }
    while done.len() < 26 {
        for l in 'A' as u8..='Z' as u8 {
            if req.get(&l).unwrap_or(&Vec::new()).iter().all(|x| done.contains(x)) && !done.contains(&l) {
                done.insert(l);
                print!("{}", l as char);
                break;
            }
        }
    }
    println!();
}
