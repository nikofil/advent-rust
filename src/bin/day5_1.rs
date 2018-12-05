extern crate advent_lib;
use advent_lib::read_line;

fn main() {
    let mut v = read_line().unwrap().chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < v.len() - 1 {
        while v[i] != v[i+1] && v[i].to_lowercase().to_string() == v[i+1].to_lowercase().to_string() {
            println!("{} {}", v[i], v[i+1]);
            v.remove(i);
            v.remove(i);
            if i > 0 {
                i -= 1;
            }
            println!("{} {}", v[i], v[i+1]);
        }
        i += 1;
    }
    println!("{}", v.len());
}
