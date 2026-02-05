use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut sum = n;
    let mut cur = n;
    let mut i = 0;
    while k > sum {
        cur += 1;
        sum += cur;
        i += 1;
    }

    println!("{}", i);
}
