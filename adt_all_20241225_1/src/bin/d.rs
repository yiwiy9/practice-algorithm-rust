use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/adt_all_20241225_1/tasks/abc376_b
/// https://atcoder.jp/contests/adt_all_20241225_1/editorial/11192
/// 円環の移動を簡潔に場合分けする視点を持つ
///
/// 1 以上 N 以下の整数 from,to,ng が与えられる。
/// 円環上でパーツ from からパーツ to まで、パーツ ng を通らないように移動したいとき、
/// 移動回数の最小値は？（ただし、from!=ng, to!=ng が保証される）
///
/// from,to は入れ替えてしまっても問題ないことに着目して、場合分けの数を減らす
/// => 常に from≤to を満たすようにする
///
/// あとは ng がどこにあるかが問題ですが、これは本質的には 2 通りしかありません
/// => from<ng<to である場合とそれ以外の場合です。
///
/// 前者については反時計回り、後者については時計回りに移動することが最適となる。
fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    }

    let num_move = |from: usize, to: usize, ng: usize| {
        let mut from = from;
        let mut to = to;
        if from > to {
            std::mem::swap(&mut from, &mut to);
        }

        if from <= ng && ng <= to {
            from + n - to
        } else {
            to - from
        }
    };

    let mut l = 0;
    let mut r = 1;
    let mut ans = 0;
    for &(h, t) in &ht {
        match h {
            'L' => {
                ans += num_move(l, t, r);
                l = t;
            }
            'R' => {
                ans += num_move(r, t, l);
                r = t;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
