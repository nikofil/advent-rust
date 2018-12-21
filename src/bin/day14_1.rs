fn main() {
    let r = [0usize, 8, 4, 6, 0, 1];
    let mut v = vec![3usize, 7];
    let mut x = 0usize;
    let mut y = 1usize;
    let mut i = 0;
    loop {
        i += 1;
        let nxt = v[x] + v[y];
        if nxt >= 10 {
            v.push(nxt / 10);
        }
        v.push(nxt % 10);
        x = (x + 1 + v[x]) % v.len();
        y = (y + 1 + v[y]) % v.len();
        let l = v.len();
        if l > 10 && (v[l-10..l] == r) {

        }
    }
    println!("{}", i);
}
