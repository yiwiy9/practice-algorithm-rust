use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut k = 0;
    while n / 2 > 0 {
        k += 1;
        n /= 2;
    }

    println!("{}", k);
}
