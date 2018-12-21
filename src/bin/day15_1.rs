extern crate advent_lib;
extern crate core;

use advent_lib::read_line;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;
use core::fmt::Write;
use std::ops::Deref;
use std::ops::DerefMut;
use std::cmp::Ordering;
use std::process::exit;

trait Player {
    fn is_enemy(&self, &Player) -> bool;
    fn is_elf(&self) -> bool;
    fn get_hp(&self) -> RefMut<i32>;
    fn dmg(&self, x: i32) -> bool {
        let mut hp = self.get_hp();
        *hp -= x;
        *hp <= 0
    }
}

impl PartialEq<Player> for Player {
    fn eq(&self, other: &Player) -> bool {
        *self.get_hp() == *other.get_hp()
    }
}

impl PartialOrd<Player> for Player {
    fn partial_cmp(&self, other: &Player) -> Option<Ordering> {
        Some(self.get_hp().cmp(&*other.get_hp()))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Player {}

enum T {
    Empty,
    Wall,
    Player(Box<dyn Player>),
}

impl T {
    fn empty(&self) -> bool {
        match self {
            T::Empty => true,
            _ => false,
        }
    }
}

impl Display for T {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char(match self {
            T::Empty => '.',
            T::Wall => '#',
            T::Player(p) => if p.is_elf() { 'E' } else { 'G' },
        })?;
        Ok(())
    }
}

struct Goblin {
    hp: RefCell<i32>,
}

impl Goblin {
    fn new() -> Box<Goblin> {
        Box::new(Goblin { hp: RefCell::new(200) })
    }
}

impl Player for Goblin {
    fn is_enemy(&self, o: &Player) -> bool {
        o.is_elf()
    }

    fn is_elf(&self) -> bool {
        false
    }

    fn get_hp(&self) -> RefMut<i32> {
        self.hp.borrow_mut()
    }
}

struct Elf {
    hp: RefCell<i32>,
}

impl Elf {
    fn new() -> Box<Elf> {
        Box::new(Elf { hp: RefCell::new(200) })
    }
}

impl Player for Elf {
    fn is_enemy(&self, o: &Player) -> bool {
        !o.is_elf()
    }

    fn is_elf(&self) -> bool {
        true
    }

    fn get_hp(&self) -> RefMut<i32> {
        self.hp.borrow_mut()
    }
}

fn main() {
    let mut v = Board(Vec::new());
    let mut elf = HashSet::new();
    let mut gob = HashSet::new();
    let mut y = 0usize;
    while let Some(x) = read_line() {
        v.push(x.chars().enumerate().map(|(x, c)| {
            match c {
                '#' => T::Wall,
                '.' => T::Empty,
                'E' => {
                    elf.insert((x, y));
                    T::Player(Elf::new())
                },
                'G' => {
                    gob.insert((x, y));
                    T::Player(Goblin::new())
                },
                _ => panic!()
            }
        }).collect::<Vec<T>>());
        y += 1;
    }
    let mx = v[0].len();
    let my = v.len();
    for ir in 0.. {
        let mut todo = Vec::new();
        for (x, y) in &elf {
            todo.push((*x, *y));
        }
        for (x, y) in &gob {
            todo.push((*x, *y));
        }
        todo.sort_by(|(x1, y1), (x2, y2)| {
            let c1 = y1.cmp(y2);
            match c1 {
                Ordering::Equal => x1.cmp(x2),
                _ => c1,
            }
        });
        let mut tonot = HashSet::new();
        for (x, y) in todo {
            if tonot.contains(&(x, y)) {
                continue;
            }
            if elf.is_empty() || gob.is_empty() {
                let f = |x: &(usize, usize)| {
                    if let T::Player(p) = v.at(x.0, x.1) {
                        let h = p.get_hp();
                        println!("{}", *h);
                        *h
                    } else {
                        panic!()
                    }
                };
                let es = elf.iter().map(&f).sum::<i32>();
                let gs = gob.iter().map(&f).sum::<i32>();
                println!("done after {} w hp {}", ir, es+gs);
                println!("score {}", ir*(es+gs));
                exit(0);
            }
            let mut todo = None;
            let mut todmg = None;
            match v.at(x, y) {
                T::Player(b) => {
                    let cl =
                        if b.is_elf() {
                            closest(x, y, &gob, &v)
                        } else {
                            closest(x, y, &elf, &v)
                        };
                    cl.map(|(x1, y1)| {
                        if x1 == x && y1 == y {
                            todmg = setatk(x, y, &v, &b);
                        } else {
                            todo = Some((x, y, x1, y1));
                        }
                    });
                },
                _ => ()
            }
            todo.map(|(x, y, x1, y1)| {
                let p = v.take(x, y);
                let (x2, y2) = path(x, y, x1, y1, mx, my, &v);
                if p.is_elf() {
                    elf.remove(&(x, y));
                    elf.insert((x2, y2));
                } else {
                    gob.remove(&(x, y));
                    gob.insert((x2, y2));
                }
                if x1 == x2 && y1 == y2 {
                    todmg = setatk(x1, y1, &v, &p);
                }
                v.set(x2, y2, p);
            });
            todmg.map(|(x1, y1, d)| {
                let p = v.take(x1, y1);
                if p.dmg(d) {
                    elf.remove(&(x1, y1));
                    gob.remove(&(x1, y1));
                    tonot.insert((x1, y1));
                } else {
                    v.set(x1, y1, p);
                }
            });
        }
        println!("{}{} over", &v, ir);
    }
}

fn setatk(x: usize, y: usize, v: &Board, b: &Box<Player>) -> Option<(usize, usize, i32)> {
    vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
        .iter()
        .filter(|(x, y)| if let T::Player(p) = v.at(*x, *y) { p.is_enemy(&**b) } else { false })
        .min_by(|(x1, y1), (x2, y2)|
            if let T::Player(p1) = v.at(*x1, *y1) {
                if  let T::Player(p2) = v.at(*x2, *y2) {
                    let cm = p1.cmp(p2);
                    match cm {
                        Ordering::Equal => {
                            let cm1 = y1.cmp(y2);
                            match cm1 {
                                Ordering::Equal => x1.cmp(x2),
                                _ => cm1,
                            }
                        },
                        _ => cm,
                    }
                } else { panic!() }
            } else { panic!() }
        ).map(|(x1, y1)| {
            (*x1, *y1, 3)
        })
}

struct Board(Vec<Vec<T>>);

impl Board {
    fn at(&self, x: usize, y: usize) -> &T {
        &self.0[y][x]
    }

    fn take(&mut self, x: usize, y: usize) -> Box<Player> {
        let res = self.0[y].remove(x);
        self.0[y].insert(x, T::Empty);
        if let T::Player(p) = res {
            p
        } else {
            panic!()
        }
    }

    fn set(&mut self, x: usize, y: usize, p: Box<Player>) {
        self.0[y][x] = T::Player(p);
    }
}

impl Deref for Board {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for i in &self.0 {
            let mut strs = Vec::new();
            for j in i {
                j.fmt(f)?;
                if let T::Player(p) = j {
                    strs.push(format!(" {}({})", if p.is_elf() { 'E' } else { 'G' }, *p.get_hp()));
                }
            }
            for i in &strs {
                f.write_str(i);
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn around(v: &HashSet<(usize, usize)>, b: &Board) -> HashSet<(usize, usize)> {
    v.iter().flat_map(|(x, y)| {
        let x = *x;
        let y = *y;
        let mut r = HashSet::new();
        if b.at(x, y-1).empty() {
            r.insert((x, y-1));
        }
        if b.at(x-1, y).empty() {
            r.insert((x-1, y));
        }
        if b.at(x+1, y).empty() {
            r.insert((x+1, y));
        }
        if b.at(x, y+1).empty() {
            r.insert((x, y+1));
        }
        r
    }).collect::<HashSet<(usize, usize)>>()
}

fn closest(x: usize, y: usize, v: &HashSet<(usize, usize)>, b: &Board) -> Option<(usize, usize)> {
    v.iter().map(|(x1, y1)| (*x1, *y1)).filter(|(x1, y1)| {
        (x == *x1 && (y == *y1-1 || y == *y1+1)) || (y == *y1 && (x == *x1-1 || x == *x1+1))
    }).next().map(|_| (x, y)).or_else(|| {
        let mut front = Vec::new();
        let mut vis = HashSet::new();
        let v = around(v, b);
        front.push((x, y));
        let mut r = None;
        'out: loop {
            let mut frontn = Vec::new();
            if front.is_empty() {
                break;
            }
            for (x, y) in front {
                if !vis.insert((x, y)) {
                    continue;
                }
                if v.contains(&(x, y)) {
                    r = Some((x, y));
                    break 'out;
                }
                if b.at(x, y - 1).empty() {
                    frontn.push((x, y - 1));
                }
                if b.at(x - 1, y).empty() {
                    frontn.push((x - 1, y));
                }
                if b.at(x + 1, y).empty() {
                    frontn.push((x + 1, y));
                }
                if b.at(x, y + 1).empty() {
                    frontn.push((x, y + 1));
                }
            }
            frontn.sort_by(|(x1, y1), (x2, y2)| {
                let c1 = y1.cmp(y2);
                match c1 {
                    Ordering::Equal => x1.cmp(x2),
                    _ => c1,
                }
            });
            front = frontn;
        }
        r
    })
}

fn path(x1: usize, y1: usize, x: usize, y: usize, mx: usize, my: usize, v: &Board) -> (usize, usize) {
    let mut p = Vec::new();
    let mut each = Vec::new();
    let mut cx = 0;
    let mut cy = 0;
    each.resize(mx, 999999);
    p.resize(my, each.clone());
    let mut front = VecDeque::new();
    front.push_front((x, y, 0));
    while let Some((x, y, cur)) = front.pop_back() {
        if x == x1 && y == y1 {
            let vv: Vec<(usize, usize)> = vec![(x1, y1-1), (x1-1, y1), (x1+1, y1), (x1, y1+1)];
            let res = *vv.iter().min_by(|(x1, y1), (x2, y2)| p[*y1][*x1].cmp(&p[*y2][*x2])).unwrap();
            cx = res.0;
            cy = res.1;
            break;
        }
        p[y][x] = cur;
        if v.at(x, y-1).empty() && p[y-1][x] > cur + 1 {
            front.push_front((x, y-1, cur+1));
        }
        if v.at(x-1, y).empty() && p[y][x-1] > cur + 1 {
            front.push_front((x-1, y, cur+1));
        }
        if v.at(x+1, y).empty() && p[y][x+1] > cur + 1 {
            front.push_front((x+1, y, cur+1));
        }
        if v.at(x, y+1).empty() && p[y+1][x] > cur + 1 {
            front.push_front((x, y+1, cur+1));
        }
    }
    (cx, cy)
}
