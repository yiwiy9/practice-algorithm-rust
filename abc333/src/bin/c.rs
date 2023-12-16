use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let a: Vec<usize> = vec![
        1,
        11,
        111,
        1111,
        11111,
        111111,
        1111111,
        11111111,
        111111111,
        1111111111,
        11111111111,
        111111111111,
    ];

    let mut b = vec![];
    for &i in &a {
        for &j in &a {
            for &k in &a {
                b.push(i + j + k);
            }
        }
    }

    let mut c = b.iter().unique().collect::<Vec<_>>();
    c.sort();

    println!("{}", c[n - 1]);
}
