use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans: usize = 0;
    for left in 0..n {
        for diff in 1..=n {
            if left + diff > n {
                break;
            }

            let mut cnt = 1;
            let mut i = left + diff;
            while i < n {
                if h[i] != h[left] {
                    break;
                }
                cnt += 1;
                i += diff;
            }

            ans = ans.max(cnt);
        }
    }

    println!("{}", ans);
}
