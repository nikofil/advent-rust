extern crate advent_lib;
extern crate regex;
use advent_lib::read_line;

fn main() {
    let re = regex::Regex::new(r"<([ -]*\d*), *([ -]*\d*)>").unwrap();
    let mut v = Vec::new();
    while let Some(x) = read_line() {
        let mut caps = re.captures_iter(&x);
        println!("{}", x);
        let x = (0..2).map(|_| {
            let y = caps.next().unwrap();
            (1..=2).map(|i| y[i].trim().parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();
        v.push(x);
    }
    let mut min = 999999999;
    for ii in 1.. {
        for x in v.iter_mut() {
            x[0][0] += x[1][0];
            x[0][1] += x[1][1];
        }
        let x = v.iter().map(|x| x[0][0]).min().unwrap();
        let y = v.iter().map(|x| x[0][1]).min().unwrap();
        let xx = v.iter().map(|x| x[0][0]).max().unwrap();
        let yy = v.iter().map(|x| x[0][1]).max().unwrap();
        if xx - x < min {
            min = xx - x;
            if min < 100 {
                (y..=yy).for_each(|y| {
                    (x..=xx).for_each(|x| {
                        let is = v.iter().any(|p| p[0][0] == x && p[0][1] == y);
                        print!("{}", if is { '#' } else { '.' });
                    });
                    println!();
                });
                println!("{}", ii);
            }
        }
    }
}

