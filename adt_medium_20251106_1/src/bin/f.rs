use itertools::Itertools;
use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        m: usize,
        s: [Chars; 3],
    }

    let mut ans = INF;
    for p in (0..3).permutations(3) {
        for num in 0..=9 {
            let target = std::char::from_digit(num, 10).unwrap();
            let mut t = 0;
            let mut ok = true;
            for &i in &p {
                if s[i].iter().all(|&c| c != target) {
                    ok = false;
                    break;
                }
                for j in 0..m {
                    let jj = (t + j) % m;
                    if s[i][jj] == target {
                        t += j + 1;
                        break;
                    }
                }
            }
            if ok {
                ans = ans.min(t);
            }
        }
    }

    println!("{}", if ans == INF { -1 } else { ans as isize - 1 });
}
