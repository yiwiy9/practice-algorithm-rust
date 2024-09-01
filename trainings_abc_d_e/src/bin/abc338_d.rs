use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc338/tasks/abc338_d
 * https://atcoder.jp/contests/abc338/editorial/9171
 * 区間加算どうやるんや？遅延セグ木か？と思ったが、差分配列の累積和で解ける（imos法）
 * （参考: https://imoz.jp/algorithms/imos_method.html）
 *
 * vi =（橋 i を封鎖したときのツアーの長さの最小値）とおいて、この vi を全ての i=1,2,…,M について求めることを考えます。
 * v の代わりにその差分配列v′（vi′ = vi − vi−1 と定義される配列）を管理することを考えると、
 * 「vl ,vl+1 ,…, vr に x を加算する」という操作は「vl′ に x を加算し、vr+1′ に −x を加算する」という操作に置き換わるので、
 * 各操作 O(1) で実行可能になります。
 * M−1 個の移動全てについてこれを行ったあと、v′ の累積和を取ることで目的の配列 v が得られます。
 *
 * 【クエリの前に加算が終了する：imos 法（区間への加算）】
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    }

    let mut v_diff = vec![0; n + 1];
    for k in 0..(m - 1) {
        if x[k] < x[k + 1] {
            let d = (x[k + 1] - x[k]) as i64;
            let rev_d = n as i64 - d;

            v_diff[x[k]] += rev_d;
            v_diff[x[k + 1]] -= rev_d;

            v_diff[x[k + 1]] += d;
            v_diff[n] -= d;
            v_diff[0] += d;
            v_diff[x[k]] -= d;
        } else {
            let rev_d = (x[k] - x[k + 1]) as i64;
            let d = n as i64 - rev_d;

            v_diff[x[k]] += rev_d;
            v_diff[n] -= rev_d;
            v_diff[0] += rev_d;
            v_diff[x[k + 1]] -= rev_d;

            v_diff[x[k + 1]] += d;
            v_diff[x[k]] -= d;
        }
    }

    let mut v = vec![0; n + 1];
    for i in 0..n {
        v[i + 1] = v[i] + v_diff[i];
    }

    println!("{}", v.iter().skip(1).min().unwrap());
}
