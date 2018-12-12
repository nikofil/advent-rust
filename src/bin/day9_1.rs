use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

type Link = Option<Rc<RefCell<Node>>>;
type Link2 = Option<Weak<RefCell<Node>>>;

struct Node {
    elem: usize,
    next: Link,
    prev: Link2,
}

impl Node {
    fn new() -> Rc<RefCell<Node>> {
        let n = Rc::new(RefCell::new(Node { elem: 0, next: None, prev: None }));
        n.borrow_mut().next = Some(Rc::clone(&n));
        n.borrow_mut().prev = Some(Rc::downgrade(&Rc::clone(&n)));
        n
    }

    fn next(&self) -> Rc<RefCell<Node>> {
        Rc::clone(self.next.as_ref().unwrap())
    }

    fn prev(&self) -> Weak<RefCell<Node>> {
        Weak::clone(self.prev.as_ref().unwrap())
    }

    fn ins(old: Rc<RefCell<Node>>, elem: usize) -> Rc<RefCell<Node>> {
        let n = Rc::new(RefCell::new(
            Node {
                elem,
                prev: Some(Rc::downgrade(&old)),
                next: old.borrow().next.as_ref().map(Rc::clone)
            }
        ));
        old.borrow_mut().next = Some(Rc::clone(&n));
        {
            let tmp = n.borrow();
            tmp.next.as_ref().unwrap().borrow_mut().prev = Some(Rc::downgrade(&n));
        }
        n
    }

    fn rm(old: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let p = old.borrow();
        let pp = p.prev.as_ref().unwrap();
        let n = old.borrow();
        let nn = n.next.as_ref().unwrap();
        let tmp = pp.upgrade().unwrap();
        tmp.borrow_mut().next = Some(Rc::clone(nn));
        nn.borrow_mut().prev = Some(Weak::clone(pp));
        Rc::clone(nn)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut cur = self.next.take();
        loop {
            let tmp = match cur {
                None => return,
                Some(x) => match Rc::try_unwrap(x) {
                    Ok(mut x) => x.borrow_mut().next.take(),
                    Err(_) => return,
                }
            };
            cur = tmp;
        }
    }
}

fn main() {
    let players = 416usize;
    let last = 71617usize;
    let mut v = vec![0;players];
    let mut cur = Node::new();
    for i in 1..=last {
        if i % 23 != 0 {
            let tmp = cur.borrow().next();
            cur = tmp;
            cur = Node::ins(cur, i);
        } else {
            let cp = (i-1) % players;
            v[cp] += i;
            (0..7).for_each(|_| {
                let tmp = cur.borrow().prev();
                cur = tmp.upgrade().unwrap();
            });
            v[cp] += cur.borrow().elem;
            let tmp = Node::rm(cur);
            cur = tmp;
        }
    }
    cur.borrow_mut().next.take();
    println!("{}", v.iter().max().unwrap());
}

