use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut memo = vec![0; 2 * n + 2];
    for (i, &a_i) in a.iter().enumerate() {
        memo[2 * (i + 1)] = memo[a_i] + 1;
        memo[2 * (i + 1) + 1] = memo[a_i] + 1;
    }

    for ans in memo.iter().skip(1) {
        println!("{}", ans);
    }
}
