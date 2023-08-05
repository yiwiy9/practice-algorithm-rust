use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut max_p = 0;
    for &p_i in p.iter().skip(1) {
        max_p = max_p.max(p_i);
    }

    println!("{}", if max_p < p[0] { 0 } else { max_p + 1 - p[0] });
}
