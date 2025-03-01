use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ok = true;
    for i in 1..n {
        if a[i] <= a[i - 1] {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
