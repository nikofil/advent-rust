extern crate advent_lib;
use advent_lib::read_line;

fn read_tree(v: &[u32]) -> (&[u32], u32) {
    let mut v = v;
    let ch = v[0] as usize;
    let m = v[1] as usize;
    let mut res = 0;
    v = &v[2..];
    for _ in 0..ch {
        let next = read_tree(v);
        v = next.0;
        res += next.1;
    }
    for i in 0..m {
        res += v[i];
    }
    v = &v[m..];
    (v, res)
}

fn main() {
    if let Some(x) = read_line() {
        let vv: Vec<&str> = x.split(' ').collect::<Vec<&str>>();
        let v = vv.iter().map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        println!("{}", read_tree(&v.as_slice()).1);
    }
}

