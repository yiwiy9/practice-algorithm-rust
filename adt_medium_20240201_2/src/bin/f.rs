use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize,usize); n],
    }

    ab.sort();
    let mut per_day = ab.iter().map(|(_, b)| *b).sum::<usize>();
    if per_day <= k {
        println!("{}", 1);
        return;
    }

    for (a, b) in ab {
        per_day -= b;
        if per_day <= k {
            println!("{}", a + 1);
            return;
        }
    }
}
