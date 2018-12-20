extern crate advent_lib;
use advent_lib::read_line;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashSet;
use std::collections::VecDeque;

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

enum T {
    Empty,
    Wall,
    Player(Box<dyn Player>),
    Step(usize),
}

impl T {
    fn empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
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
    let mut v = Vec::new();
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
    loop {
        for y in 0..my {
            for x in 0..mx {
                match &v[y][x] {
                    T::Player(b) => {
                        let mut cl;
                        if b.is_elf() {
                            cl = closest(x, y, &gob, &v);
                        } else {
                            cl = closest(x, y, &elf, &v);
                        }
                        println!("Closest to {} {} is {} {}", x, y, cl.unwrap().0, cl.unwrap().1);
                    },
                    _ => ()
                }
            }
        }
    }
}

fn closest(x: usize, y: usize, v: &HashSet<(usize, usize)>, b: &Vec<Vec<T>>) -> Option<(usize, usize)> {
    let mut front = VecDeque::new();
    front.push_front((x, y));
    let mut r = None;
    while let Some((x, y)) = front.pop_back() {
        if v.contains(&(x, y)) {
            r = Some((x, y));
            break;
        }
        if b[y-1][x].empty() {
            front.push_front((y-1, x));
        }
        if b[y][x-1].empty() {
            front.push_front((y, x-1));
        }
        if b[y][x+1].empty() {
            front.push_front((y, x+1));
        }
        if b[y+1][x].empty() {
            front.push_front((y+1, x));
        }
    }
    r
}
