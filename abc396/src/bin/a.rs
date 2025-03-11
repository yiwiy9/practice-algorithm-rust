use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ok = false;
    for i in 0..n - 2 {
        if a[i] == a[i + 1] && a[i + 1] == a[i + 2] {
            ok = true;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
