use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp_1 = vec![vec![INF; 2]; n];
    dp_1[0][1] = a[0];
    for i in 1..n {
        dp_1[i][0] = dp_1[i - 1][1];
        dp_1[i][1] = dp_1[i - 1][0].min(dp_1[i - 1][1]) + a[i];
    }

    let mut dp_2 = vec![vec![INF; 2]; n];
    dp_2[0][0] = 0;
    for i in 1..n {
        dp_2[i][0] = dp_2[i - 1][1];
        dp_2[i][1] = dp_2[i - 1][0].min(dp_2[i - 1][1]) + a[i];
    }

    println!("{}", dp_1[n - 1][0].min(dp_1[n - 1][1]).min(dp_2[n - 1][1]));
}
