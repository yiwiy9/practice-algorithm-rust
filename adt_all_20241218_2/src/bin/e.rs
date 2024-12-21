use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }

    let mut cnt = 0;

    let k_minus = k - 1;

    // next_permutation()
    for t in s.iter().permutations(n).unique() {
        let mut ok = true;

        for i in 0..n - k_minus {
            let mut has_p = true;
            for diff in 0..k_minus {
                let left = i + diff;
                let right = i + k_minus - diff;
                if t[left] != t[right] {
                    has_p = false;
                    break;
                }
            }
            if has_p {
                ok = false;
                break;
            }
        }

        if ok {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
