use ac_library::{Additive, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::{BTreeMap, BTreeSet};
use superslice::*;

const INF: usize = 1 << 60;

/// https://atcoder.jp/contests/abc449/tasks/abc449_e
/// https://atcoder.jp/contests/abc449/editorial/17253
/// https://atcoder.jp/contests/abc449/editorial/17125
///
/// 中間状態を持ちすぎていて重そうと感じたら、クエリ先読みを疑う。
/// 今回は各段の累積配列を全部持つのではなく、
/// クエリを upper ごとに先読みして前から処理すればよかった。
///
/// acc_vec[upper][idx] を見たら、
/// 「配列アクセス」ではなく「prefix 集合の idx 番目取得」と言い換える。
/// そこまで言い換われば Segtree / BIT を疑う。
fn wa_main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        q: usize,
        x: [Usize1; q],
    }

    let mut cnt = vec![0; m + 1];
    for &a_i in &a {
        cnt[a_i] += 1;
    }

    let mut cnt_num_map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for i in 1..=m {
        cnt_num_map.entry(cnt[i]).or_default().insert(i);
    }

    // ここまでは良い:
    // 値を出現回数ごとにまとめる整理は本質に近い。
    let cnt_num_vec = cnt_num_map.into_iter().collect_vec();
    let k = cnt_num_vec.len();

    // これは残してよい:
    // 各段の境界長を持てば、あとで upper を二分探索できる。
    let mut acc_index = vec![0; k];

    // 気づきポイント:
    // 各段の完成済み配列を全部持ち始めた時点で「持ちすぎ」を疑う。
    // 後で 1 要素引きたいだけなのに、各段の全体を保存している。
    let mut acc_vec = vec![vec![]; k];

    for kk in 0..k - 1 {
        let (kk_cnt, _) = cnt_num_vec[kk];
        let (kk_plus_cnt, _) = cnt_num_vec[kk + 1];

        let kk_set = cnt_num_vec[kk].1.clone();
        let mut kk_vec = kk_set.into_iter().collect_vec();

        // 気づきポイント:
        // 前段の累積配列をコピーして次段を作っている。
        // 「累積情報を使いたい」だけなのに、
        // 「各段の完成形を毎回実体として持つ」設計になっている。
        kk_vec.extend_from_slice(&acc_vec[kk]);
        let kk_vec_len = kk_vec.len();

        // 気づきポイント:
        // この sort 済み配列を段ごとに保存するのが重さの原因。
        // 本当に欲しいのは配列全体ではなく、後で使う順位 1 個だけ。
        acc_vec[kk + 1] = kk_vec;
        acc_vec[kk + 1].sort();

        acc_index[kk + 1] = acc_index[kk] + ((kk_plus_cnt - kk_cnt) * kk_vec_len);
    }

    for &x_i in &x {
        if x_i < n {
            println!("{}", a[x_i]);
            continue;
        }

        let mut x_i = x_i - n;

        let upper = acc_index.upper_bound(&x_i);

        if upper < acc_index.len() {
            x_i -= acc_index[upper - 1];
            let idx = x_i % acc_vec[upper].len();

            // 最大の気づきポイント:
            // acc_vec[upper][idx] は単なる配列アクセスではなく、
            // 「upper 段までの集合の idx 番目取得」。
            //
            // ここまで言い換われば、
            // - 各段の配列全保存は不要
            // - クエリを upper ごとに先読み
            // - 前から 1 回だけ構築
            // - Segtree / BIT で idx 番目取得
            // という公式解の形を疑える。
            println!("{}", acc_vec[upper][idx]);
        } else {
            x_i -= acc_index[acc_index.len() - 1];
            println!("{}", (x_i % m) + 1);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
        q: usize,
        x: [usize; q],
    }

    let mut cnt = vec![0; m];
    for &v in &a {
        cnt[v] += 1;
    }

    // s[i] = (出現回数, 値)
    let mut s = (0..m).map(|i| (cnt[i], i)).collect_vec();
    s.sort();

    // r[k] = k 段目まで終わった直後の長さ
    let mut r = vec![0; m + 1];
    r[0] = n;
    for i in 0..m - 1 {
        r[i + 1] = r[i] + (i + 1) * (s[i + 1].0 - s[i].0);
    }
    r[m] = INF;

    let mut ans = vec![0; q];
    let mut queries = vec![]; // (k, v, query_id)

    for (qi, &x_i) in x.iter().enumerate() {
        if x_i <= n {
            ans[qi] = a[x_i - 1] + 1;
            continue;
        }

        // r[ok] >= x_i となる最小の ok
        let ok = r.lower_bound(&x_i);

        // ok-1 段まででの長さを使う
        let mut v = x_i - (r[ok - 1] + 1);
        v %= ok;

        queries.push((ok, v, qi));
    }

    queries.sort();

    let mut segtree = Segtree::<Additive<usize>>::new(m);
    let mut idx = 0;

    for (ok, v, qi) in queries {
        while idx < ok {
            segtree.set(s[idx].1, 1);
            idx += 1;
        }

        // sum(0..pos) <= v を満たす最大 pos を返す
        // => 小さい方から v 番目(0-indexed)の値
        ans[qi] = segtree.max_right(0, |&sum| sum <= v) + 1;
    }

    for v in ans {
        println!("{v}");
    }
}
