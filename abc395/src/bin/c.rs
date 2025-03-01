use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = n + 1;
    let mut before_index = vec![n; 1_000_001];
    for (i, &a_i) in a.iter().enumerate() {
        if before_index[a_i] != n {
            ans = ans.min(i - before_index[a_i] + 1);
        }
        before_index[a_i] = i;
    }

    println!("{}", if ans == n + 1 { -1 } else { ans as isize });
}
