fn main() {
    let s = 6303;
    let v = (1..=300).map(|y|
        (1..=300).map(|x| {
            let r = x + 10;
            let p = r * r * y + r * s;
            let h = (p / 100) % 10;
            h - 5
        }).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let r = (0..298).flat_map(|y|
        (0..298).map(|x|
            ((0..3).map(|dy|
                (0..3).map(|dx|
                    v[y + dy][x + dx]
                ).sum::<i32>()
            ).sum(), x + 1, y + 1)
        ).collect::<Vec<(i32, usize, usize)>>()
    ).collect::<Vec<(i32, usize, usize)>>();
    let (_, x, y) = r.iter().max_by(|(v1, ..), (v2, ..)| v1.cmp(v2)).unwrap();
    println!("{},{}", x, y);
}
