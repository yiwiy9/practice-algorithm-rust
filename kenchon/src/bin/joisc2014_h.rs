use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/joisc2014/tasks/joisc2014_h
/// https://www2.ioi-jp.org/camp/2014/2014-sp-tasks/2014-sp-d3.pdf
/// https://drken1215.hatenablog.com/entry/2020/11/28/202800
/// => 累積和から逸脱できず手も足も出なかった
///
/// 足がかりとして、文字列が3種類しかないことに着目すると見えてくる
///
/// 仮に、文字列が2種類だと考えると、
/// P[i] := 最初の i 文字についての、(A の個数) - (B の個数)
/// このとき、「区間 [l, r) が条件を満たす」 <=>  P[l] = P[r]
///
/// 同様にして、３種類の場合も考えると、
/// P[i] := 最初の i 文字についての、("O" の個数) - ("J" の個数)
/// Q[i] := 最初の i 文字についての、("I" の個数) - ("J" の個数)
/// このとき、「区間 [l, r) が条件を満たす」 <=>  P[l] = P[r] かつ Q[l] = Q[r]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut map = BTreeMap::new();
    let mut j_cnt = 0;
    let mut o_cnt = 0;
    let mut i_cnt = 0;
    map.insert((0, 0), vec![0]);
    for (i, &c) in s.iter().enumerate() {
        if c == 'J' {
            j_cnt += 1;
        } else if c == 'O' {
            o_cnt += 1;
        } else {
            i_cnt += 1;
        }
        map.entry((o_cnt - j_cnt, i_cnt - j_cnt))
            .or_insert(vec![])
            .push(i + 1);
    }

    let mut ans = 0;
    for same_values in map.values() {
        if same_values.len() > 1 {
            ans = ans.max(same_values.last().unwrap() - same_values.first().unwrap());
        }
    }

    println!("{}", ans);
}
