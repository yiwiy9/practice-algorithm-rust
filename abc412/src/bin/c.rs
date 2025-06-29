use proconio::input;
use superslice::*;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut s: [usize; n],
        }
        let mut cur = s[0];
        let last = s[n - 1];
        s.sort();

        let mut ans = 1;
        loop {
            let idx = s.upper_bound(&(2 * cur));
            ans += 1;
            if idx == 0 {
                ans = -1;
                break;
            }
            if s[idx - 1] >= last {
                break;
            }
            if s[idx - 1] <= cur {
                ans = -1;
                break;
            }
            cur = s[idx - 1];
        }
        println!("{}", ans);
    }
}
