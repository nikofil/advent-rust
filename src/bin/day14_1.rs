fn main() {
    let r: usize = 084601;
    let mut v = vec![3usize, 7];
    let mut x = 0usize;
    let mut y = 1usize;
    while v.len() < r+10 {
        let nxt = v[x] + v[y];
        if nxt >= 10 {
            v.push(nxt / 10);
        }
        v.push(nxt % 10);
        x = (x + 1 + v[x]) % v.len();
        y = (y + 1 + v[y]) % v.len();
    }
    println!("{:?}", &v[r..r+10]);
}
