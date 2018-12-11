extern crate advent_lib;
use advent_lib::read_line;
use std::cmp::max;

fn main() {
    let mut v: Vec<(i32, i32)> = Vec::new();
    let mut sc = Vec::new();
    while let Some(s) = read_line() {
        let c = s.split(',' as char).map(|x| x.trim().parse::<i32>().unwrap() as i32).collect::<Vec<i32>>();
        v.push((c[0], c[1]));
        sc.push(0);
    }
    let (mx, my) = v.iter().fold((0, 0), |(x, y), (mx, my)| (max(x, *mx), max(y, *my)));
    let mid = (mx+my)*2;
    for i in 0..=mid*2 {
        for j in 0..=mid*2 {
            let mut vv = v.iter().map(|(x,y)| (i-x-mid).abs() + (j-y-mid).abs()).enumerate().collect::<Vec<(usize, i32)>>();
            vv.sort_by(|(_i, x), (_i2, x2)| x.cmp(x2));
            if vv[0].1 != vv[1].1 {
                if i == mid*2 || j == mid*2 || i == 0 || j == 0 {
                    sc[vv[0].0] = -99999999;
                } else {
                    sc[vv[0].0] += 1;
                }
            }
        }
    }
    for i in sc.iter().enumerate() {
        println!("{} {}", i.0, i.1);
    }
    println!("{}", sc.iter().max().unwrap());
}
