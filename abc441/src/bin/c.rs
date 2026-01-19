use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    if s[k] < x {
        println!("-1");
        return;
    }

    for i in (0..k).rev() {
        if s[k] - s[i] >= x {
            println!("{}", n - i);
            return;
        }
    }
}
