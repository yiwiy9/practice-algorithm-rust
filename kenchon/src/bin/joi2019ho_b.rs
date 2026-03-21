use proconio::input;

/// https://atcoder.jp/contests/joi2019ho/tasks/joi2019ho_b
/// https://drken1215.hatenablog.com/entry/2019/02/15/014400
///
/// セグ木上のDPに飛び込んでしまった！（座圧後の重複数の扱いがダメで断念）
/// => 値やサイズに単調性があるなら、まず “厳しい方（今回は大きい方）から順に見て Greedy できないか” を疑う
/// => 大きい方の額縁から使っていくことは明白なので、あとは絵の価値順に入るだけ入れていく
fn main() {
    input! {
        n: usize,
        m: usize,
        mut sv: [(usize,usize); n],
        mut c: [usize; m],
    }

    sv.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
    c.sort_by(|a, b| b.cmp(a));

    let mut i = 0;
    let mut j = 0;
    while j < m {
        while i < n && sv[i].0 > c[j] {
            i += 1;
        }
        if i == n {
            break;
        }
        i += 1;
        j += 1;
    }

    println!("{}", j);
}
