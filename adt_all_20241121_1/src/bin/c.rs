use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize,usize,usize,usize); n],
    }

    let mut set = std::collections::HashSet::new();
    for &(a, b, c, d) in &abcd {
        for i in a..b {
            for j in c..d {
                set.insert((i, j));
            }
        }
    }

    println!("{}", set.len());
}
