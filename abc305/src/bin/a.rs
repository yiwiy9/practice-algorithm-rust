use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let n_div = n % 5;

    if n_div < 3 {
        println!("{}", n - n_div);
    } else {
        println!("{}", n + (5 - n_div));
    }
}
