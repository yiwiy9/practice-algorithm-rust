use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    if d == 0 {
        let mut b = a;
        b.sort();
        b.dedup();
        println!("{}", n - b.len());
        return;
    }

    let mut mod_d_vec: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); d];
    for &a_i in &a {
        *mod_d_vec[a_i % d].entry(a_i / d).or_default() += 1;
    }

    let mut ans: usize = 0;
    for mod_d_map in &mod_d_vec {
        if mod_d_map.is_empty() {
            continue;
        }

        // 偶奇の場合分けのみだと、○xx○xx○が最適な場合に対応できない
        // そのため、DPで解く
        // 初回と同じ間違いをしている。ええ加減にしてくれ

        // let mut even_sum = 0;
        // let mut odd_sum = 0;
        // let mut continuous_cnt = 0;
        // let mut before_div: usize = 1 << 60;
        // for (&div, &cnt) in mod_d_map {
        //     if before_div + 1 != div {
        //         ans += odd_sum.min(even_sum);

        //         even_sum = 0;
        //         odd_sum = 0;
        //         continuous_cnt = 0;
        //     }

        //     if continuous_cnt % 2 == 0 {
        //         even_sum += cnt;
        //     } else {
        //         odd_sum += cnt;
        //     }

        //     continuous_cnt += 1;
        //     before_div = div;
        // }
        // ans += odd_sum.min(even_sum);

        let mut prev2 = 0; // dp[i - 2]
        let mut prev1 = 0; // dp[i - 1]
        for (&div, &cnt) in mod_d_map {
            let next = (prev1 + *mod_d_map.get(&(div + 1)).unwrap_or(&0)).min(prev2 + cnt);
            prev2 = prev1;
            prev1 = next;
        }
        ans += prev1;
    }

    println!("{}", ans);
}
