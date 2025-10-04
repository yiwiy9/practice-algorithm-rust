use proconio::input;

fn main() {
    input! {
        n: usize,
        rc: [(usize,usize); n],
    }

    let min_r = rc.iter().map(|&(r, _)| r).min().unwrap_or(0);
    let max_r = rc.iter().map(|&(r, _)| r).max().unwrap_or(0);
    let min_c = rc.iter().map(|&(_, c)| c).min().unwrap_or(0);
    let max_c = rc.iter().map(|&(_, c)| c).max().unwrap_or(0);

    let longest = (max_r - min_r).max(max_c - min_c);

    println!("{}", (longest + 1) / 2);
}
