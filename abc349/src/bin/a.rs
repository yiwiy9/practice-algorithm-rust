use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n-1],
    }

    println!("{}", (-1) * a.iter().sum::<i32>());
}
