use proconio::input;

/// https://atcoder.jp/contests/abc428/tasks/abc428_d
/// https://atcoder.jp/contests/abc428/editorial/14245
/// 性質を大方理解した後の動きが弱かった
/// →もう少しやろうとすることを言語化すべき
/// →そうすると自ずとその問題を解くために必要な最適（簡単）な式が浮かび上がるはず
///
/// 今回で言うと、「C+Dが持つ桁分変動」と書いたのを
/// 1 ≤ x ≤ D
/// 10^(d−1) − C ≤ x ≤ 10^d − 1 − C
/// の共通部分と定式化すべき
///
/// 特に「10^d」と書くのはいつもやっていない気がするので今後はまず置くべき
fn main() {
    input! {
        t: usize,
        cd: [(usize,usize); t],
    }

    // f(C, C) <= N^2
    // f(C, C+D) >= M^2
    // C+Dが持つ桁分変動

    for &(c, d) in &cd {
        let mut ans = 0;

        let mut x_min = 1;
        let mut x_max = 9;
        let mut c_shift = 10; // 1桁目から考えられる→簡単

        while x_min <= c + d {
            let l = x_min.max(c + 1);
            let r = x_max.min(c + d);

            if l <= r {
                let v_l = c * c_shift + l;
                let v_r = c * c_shift + r;
                ans += u64_floor_sqrt(v_r as u64) - u64_floor_sqrt(v_l as u64 - 1);
            }

            x_min *= 10;
            x_max = (x_max + 1) * 10 - 1;
            c_shift *= 10;
        }

        println!("{}", ans);
    }
}

/// f64の仮数部は53bitなので、nが2^53を超えると誤差が生まれる
/// そのため、u64の平方根を求める関数を自前で実装する必要がある
/// https://rsk0315.hatenablog.com/entry/2023/11/07/221428
pub fn u64_floor_sqrt(n: u64) -> u64 {
    let tmp = (n as f64).sqrt() as u64;
    let tmp_m1 = tmp.saturating_sub(1);
    if tmp_m1 * (tmp_m1 + 2) < n {
        tmp
    } else {
        tmp_m1
    }
}
