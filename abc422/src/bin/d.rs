use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut cur = vec![k];
    for _ in 0..n {
        let mut next = vec![];
        for &c in &cur {
            if c % 2 == 0 {
                next.push(c / 2);
                next.push(c / 2);
            } else {
                next.push(c / 2);
                next.push(c / 2 + 1);
            }
        }
        cur = next;
    }

    println!("{}", if cur.iter().all_equal() { 0 } else { 1 });
    println!("{}", cur.iter().join(" "));
}
