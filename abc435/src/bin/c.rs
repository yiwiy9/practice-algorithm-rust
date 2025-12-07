use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut remain = a[0] - 1;
    let mut i = 1;
    while remain > 0 && i < n {
        remain = remain.max(a[i]);
        remain -= 1;
        i += 1;
    }

    println!("{}", i);
}
