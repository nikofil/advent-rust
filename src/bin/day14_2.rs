fn main() {
    let r = [0usize, 8, 4, 6, 0, 1];
    let rl = r.len();
    let mut v = vec![3usize, 7];
    let mut x = 0usize;
    let mut y = 1usize;
    loop {
        let nxt = v[x] + v[y];
        if nxt >= 10 {
            v.push(nxt / 10);
            let l = v.len();
            if l > 10 && v[l-rl..l] == r {
                break;
            }
        }
        v.push(nxt % 10);
        let l = v.len();
        if l > 10 && v[l-rl..l] == r {
            break;
        }
        x = (x + 1 + v[x]) % v.len();
        y = (y + 1 + v[y]) % v.len();
    }
    println!("{}", v.len() - rl);
}
