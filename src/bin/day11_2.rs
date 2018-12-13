fn main() {
    let s = 6303;
    let v = (1..=300).map(|y|
        (1..=300).map(|x| {
            let r = x + 10;
            let p = r * r * y + r * s;
            let h = (p / 100) % 10;
            h - 5
        }).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut vv = (0..=300).map(|_| vec![0i32;301]).collect::<Vec<Vec<i32>>>();
    let r = (1..=300).flat_map(|sz|
        (0..=300 - sz).flat_map(|y|
            (0..=300 - sz).map(|x| {
                let sm = (1..sz).map(|i| v[y + i][x] + v[y][x + i]).sum::<i32>() + v[y][x] + vv[y+1][x+1];
                vv[y][x] = sm;
                (sm, x + 1, y + 1, sz)
            }).collect::<Vec<(i32, usize, usize, usize)>>()
        ).collect::<Vec<(i32, usize, usize, usize)>>()
    ).collect::<Vec<(i32, usize, usize, usize)>>();
    let (_, x, y, z) = r.iter().max_by(|(v1, ..), (v2, ..)| v1.cmp(v2)).unwrap();
    println!("{},{},{}", x, y, z);
}
