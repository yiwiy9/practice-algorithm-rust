use proconio::input;

fn main() {
    input! {
        a: [usize; 64],
    }

    let ans = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| a_i * (1 << i))
        .sum::<usize>();
    println!("{}", ans);
}
