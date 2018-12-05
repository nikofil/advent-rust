extern crate advent_lib;
use advent_lib::read_line;
use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    while let Some(x) = read_line() {
        v.push(x);
    }
    v.sort();

    let mut guards = HashMap::new();
    let mut cur: u32 = 0;
    let mut curmin: u32 = 0;
    for i in v {
        if i.contains("begins") {
            cur = i.split(' ').collect::<Vec<&str>>()[3][1..].parse().unwrap();
            guards.entry(cur).or_insert(Vec::new());
        } else if i.contains("falls") {
            curmin = i.split(' ').collect::<Vec<&str>>()[1][3..5].parse().unwrap();
        } else {
            let lastmin: u32 = i.split(' ').collect::<Vec<&str>>()[1][3..5].parse().unwrap();
            guards.get_mut(&cur).map(|x| x.push((curmin, lastmin)));
        }
    }
    let best = guards.iter().map(|(x, y)| {
        let total = y.iter().fold(0, |a, (n0, n1)| a + n1 - n0);
        (x, total)
    }).max_by(|(_, x), (_, y)| x.cmp(y)).unwrap().0;
    guards.get(best).map(|b| {
        let mut mins = [0; 60];
        for (f, t) in b {
            for x in *f..*t {
                mins[x as usize] += 1;
            }
        }
        let min = mins
            .iter()
            .enumerate()
            .max_by(|(_, x1), (_, x2)| x1.cmp(x2))
            .unwrap()
            .0 as u32;
        println!("{}", best * min);
    });
}
