use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: [usize; n],
    }

    let mut ok = t[0] <= s;
    for i in 1..n {
        if t[i] > s + t[i - 1] {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
