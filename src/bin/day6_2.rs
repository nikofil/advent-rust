extern crate advent_lib;
use advent_lib::read_line;
use std::cmp::max;

fn main() {
    let mut v: Vec<(i32, i32)> = Vec::new();
    let mut r = 0u32;
    while let Some(s) = read_line() {
        let c = s.split(',' as char).map(|x| x.trim().parse::<i32>().unwrap() as i32).collect::<Vec<i32>>();
        v.push((c[0], c[1]));
    }
    let (mx, my) = v.iter().fold((0, 0), |(x, y), (mx, my)| (max(x, *mx), max(y, *my)));
    let mid = (mx+my)*2;
    for i in 0..=mid*2 {
        for j in 0..=mid*2 {
            let tot: i32 = v.iter().map(|(x,y)| (i-x-mid).abs() + (j-y-mid).abs()).sum();
            if tot < 10000 {
                r += 1;
            }
        }
    }
    println!("{}", r);
}
