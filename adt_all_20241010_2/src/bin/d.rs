use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize; n],
    }

    l.sort_by(|a, b| b.cmp(a));

    let l_p = l[p - 1];

    println!("{}", if t > l_p { t - l_p } else { 0 });
}
