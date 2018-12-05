extern crate advent_lib;
use advent_lib::read_line;

fn split(s: String) -> (String, usize, usize, usize, usize) {
    let x = s.split(' ').collect::<Vec<&str>>();
    let id = x[0][1..].to_string();
    let x1 = x[2][..x[2].len()-1].split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let x2 = x[3].split('x').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    (id, x1[0], x1[1], x1[0]+x2[0], x1[1]+x2[1])
}

fn main() {
    let mut v = Vec::new();
    let mut vv = Vec::new();
    v.extend((0..4000000).map(|_| 0));
    while let Some(x) = read_line() {
        vv.push(split(x));
    }
    for (_id, x0, y0, x1, y1) in &vv {
        for y in *y0..*y1 {
            for x in *x0..*x1 {
                v[y*2000 + x] += 1;
            }
        }
    }
    for (id, x0, y0, x1, y1) in vv {
        let mut nolap = true;
        for y in y0..y1 {
            for x in x0..x1 {
                v[y*2000 + x] += 1;
                if v[y*2000 + x] > 2 {
                    nolap = false;
                }
            }
        }
        if nolap {
            println!("{}", id);
        }
    }
}
