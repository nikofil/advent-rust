extern crate advent_lib;
use advent_lib::read_no_trim;

enum Node {
    V, H, LL, RR, X, Non
}

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    L=0, U=1, R=2, D=3
}

use Node::*;
use Dir::*;
use std::cmp::Ordering;
use std::collections::HashMap;

const DIRS: [Dir; 4] =  [L, U, R, D];

fn main() {
    let mut v = Vec::new();
    let mut c = HashMap::new();
    let mut y: usize = 0;
    while let Some(x) = read_no_trim() {
        v.push(x.chars().enumerate().map(|(x, cc)| match cc {
            '/' => LL,
            '\\' => RR,
            '|' => V,
            '-' => H,
            '+' => X,
            '^' => {
                c.insert((x, y), (U, 0));
                V
            },
            'v' => {
                c.insert((x, y), (D, 0));
                V
            },
            '>' => {
                c.insert((x, y), (R, 0));
                H
            },
            '<' => {
                c.insert((x, y), (L, 0));
                H
            },
            _ => Non
        }).collect::<Vec<Node>>());
        y += 1;
    }
    loop  {
        let mut keys = c.keys().map(|(x, y)| (*x, *y)).collect::<Vec<(usize, usize)>>();
        keys.sort_by(|(x1, y1), (x2, y2)| if y1.cmp(y2) == Ordering::Equal {
            x1.cmp(x2)
        } else {
            y1.cmp(y2)
        });
        for (mut x, mut y) in keys {
            if let Some((mut d, mut a)) = c.remove(&(x, y)) {
                match v[y][x] {
                    V => {
                        if d == U {
                            y -= 1;
                        } else {
                            y += 1;
                        }
                    },
                    H => {
                        if d == L {
                            x -= 1;
                        } else {
                            x += 1;
                        }
                    },
                    LL => {
                        match d {
                            L => {
                                d = D;
                                y += 1;
                            },
                            U => {
                                d = R;
                                x += 1;
                            },
                            D => {
                                d = L;
                                x -= 1;
                            },
                            R => {
                                d = U;
                                y -= 1;
                            },
                        }
                    },
                    RR => {
                        match d {
                            L => {
                                d = U;
                                y -= 1;
                            },
                            U => {
                                d = L;
                                x -= 1;
                            },
                            R => {
                                d = D;
                                y += 1;
                            },
                            D => {
                                d = R;
                                x += 1;
                            },
                        }
                    },
                    X => {
                        let nd = DIRS[(((d as i32 + a) - 1 + 4) % 4) as usize];
                        match nd {
                            U => y -= 1,
                            D => y += 1,
                            L => x -= 1,
                            R => x += 1,
                        }
                        d = nd;
                        a = (a + 1) % 3;
                    },
                    Non => panic!(),
                }
                if c.remove(&(x, y)).is_none() {
                    c.insert((x, y), (d, a));
                }
            }
        }
        if c.len() == 1 {
            let (x, y) = c.keys().next().unwrap();
            println!("{},{}", x, y);
            break;
        }
    }
}

