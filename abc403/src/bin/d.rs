use proconio::input;

const MAX_NUM: usize = 1_000_000;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0; MAX_NUM + 1];
    for i in 0..n {
        cnt[a[i]] += 1;
    }

    if d == 0 {
        println!("{}", n - cnt.iter().filter(|&&x| x > 0).count());
        return;
    }

    // 偶奇の場合分けのみだと、○xx○xx○が最適な場合に対応できない
    // そのため、DPで解く
    // let mut ans = 0;
    // for base in 0..d {
    //     let mut continuous = 0;
    //     let mut odd_sum = 0;
    //     let mut even_sum = 0;

    //     let mut num = base;

    //     while num <= MAX_NUM {
    //         if cnt[num] > 0 {
    //             continuous += 1;
    //             if continuous % 2 == 1 {
    //                 odd_sum += cnt[num];
    //             } else {
    //                 even_sum += cnt[num];
    //             }
    //         } else {
    //             if continuous > 0 {
    //                 ans += odd_sum.min(even_sum);
    //             }
    //             continuous = 0;
    //             odd_sum = 0;
    //             even_sum = 0;
    //         }

    //         num += d;
    //     }
    //     if continuous > 0 {
    //         ans += odd_sum.min(even_sum);
    //     }
    // }

    // dp[j+1] = min(dp[j] + cnt[i + d(j+1)], dp[j-1] + cnt[i + dj])
    let mut ans = 0;
    for base in 0..d {
        let mut prev2 = 0; // dp[i - 2]
        let mut prev1 = 0; // dp[i - 1]
        let mut idx = base;
        while idx + d <= MAX_NUM {
            let next = (prev1 + cnt[idx + d]).min(prev2 + cnt[idx]);
            prev2 = prev1;
            prev1 = next;
            idx += d;
        }
        ans += prev1;
    }

    println!("{}", ans);
}
