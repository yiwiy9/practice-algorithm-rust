use itertools::Itertools;
use proconio::input;

const MAX_A: usize = 1_000_005;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0; MAX_A];
    for &a_i in &a {
        cnt[a_i] += 1;
    }

    let mut divisors_cnt = vec![0; MAX_A];
    let mut num_k_gcd = vec![0; MAX_A];

    // スタート位置のループ
    for num in 1..MAX_A {
        // 間隔のループ（調和級数）
        // 1 + 1/2 + 1/3 + ... + 1/N = O(log N)
        for mul in 1..MAX_A {
            let cur_num = num * mul;
            if cur_num >= MAX_A {
                break;
            }
            divisors_cnt[num] += cnt[cur_num];
        }

        if divisors_cnt[num] < k {
            continue;
        }

        // 間隔のループ（調和級数）
        // 1 + 1/2 + 1/3 + ... + 1/N = O(log N)
        for mul in 1..MAX_A {
            let cur_num = num * mul;
            if cur_num >= MAX_A {
                break;
            }
            num_k_gcd[cur_num] = num_k_gcd[cur_num].max(num);
        }
    }

    println!("{}", a.iter().map(|&a_i| num_k_gcd[a_i]).join("\n"));
}
