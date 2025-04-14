use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut before_index = vec![-1; n + 1];
    let mut ans = 0;
    for (i, &a_i) in a.iter().enumerate() {
        let i = i as isize;
        let multiple = i - before_index[a_i];
        ans += multiple * (n as isize - i);
        before_index[a_i] = i;
    }

    println!("{}", ans);
}
