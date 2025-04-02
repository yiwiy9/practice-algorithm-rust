use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut num = vec![0; 100_000];
    for i in 0..n {
        num[a[i]] += 1;
    }

    println!(
        "{}",
        num.iter().enumerate().find(|&(_, &x)| x == 0).unwrap().0
    );
}
