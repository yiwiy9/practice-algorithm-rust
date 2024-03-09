use itertools::Itertools;
use proconio::input;

const INF: usize = 1 << 30;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut receive_times = vec![INF; n];
    for i in 0..2 * n {
        if i > 0 {
            receive_times[i % n] =
                receive_times[i % n].min(receive_times[(i - 1) % n] + s[(i - 1) % n]);
        }
        receive_times[i % n] = receive_times[i % n].min(t[i % n]);
    }

    println!("{}", receive_times.iter().join("\n"));
}
