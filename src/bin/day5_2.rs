extern crate advent_lib;
use advent_lib::read_line;

fn getl(s: String) -> usize {
    let mut v = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < v.len() - 1 {
        while v[i] != v[i+1] && v[i].to_lowercase().to_string() == v[i+1].to_lowercase().to_string() {
            v.remove(i);
            v.remove(i);
            if i > 0 {
                i -= 1;
            }
            if i >= v.len() - 1 {
                break;
            }
        }
        i += 1;
    }
    v.len()
}

fn main() {
    let s = read_line().unwrap();
    let m = ('a' as u8..='z' as u8).map(|c: u8|
        getl(s.replace(c as char, "").replace(c.to_ascii_uppercase() as char, ""))
    ).min().unwrap();
    println!("{}", m);
}
