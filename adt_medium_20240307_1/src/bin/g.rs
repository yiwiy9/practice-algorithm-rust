use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

/**
 * https://atcoder.jp/contests/adt_medium_20240307_1/tasks/abc338_d
 * https://atcoder.jp/contests/adt_medium_20240307_1/editorial/9171
 *
 * 円になっている
 * => xからyへ行くとき、+と-の２通りの方法があって、封鎖する橋がない方を通れば最短経路になる
 * => 各移動において、各封鎖する橋ごとのツアー最小値を考えたとき、+と-のうちの封鎖する橋が入っていない方の長さをそれぞれ足し上げれば良い
 * => ただし、移動ごとに封鎖する橋でループしていると間に合わないので、
 *    +と-に属するかによって各封鎖する橋に加算する長さは等しいので、差分配列で考えて、区間に対する加算を行う
 * => あとは、この配列の累積和を取れば、各封鎖する橋のツアー最小値を求められる
 *
 * 差分配列v'（v'i=vi−vi−1と定義される配列）を管理することを考えると、
 * 「vl,vl+1,…,vr に x を加算する」という操作は「v'l に x を加算し、v'r+1 に −x を加算する」という操作に置き換わる
 *
 * この手法は俗に imos 法と呼ばれる
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    }

    let mut v = vec![0; n + 1];

    let dist = |from: usize, to: usize| {
        if from <= to {
            (to - from) as i64
        } else {
            (n + to - from) as i64
        }
    };

    let mut add = |from: usize, to: usize, num: i64| {
        if from <= to {
            v[from] += num;
            v[to] -= num;
        } else {
            v[from] += num;
            v[n] -= num;
            v[0] += num;
            v[to] -= num;
        }
    };

    for i in 0..m - 1 {
        add(x[i], x[i + 1], dist(x[i + 1], x[i]));
        add(x[i + 1], x[i], dist(x[i], x[i + 1]));
    }

    let mut ans = INF;
    for i in 0..n {
        v[i + 1] += v[i];
        ans = ans.min(v[i]);
    }

    println!("{}", ans);
}
