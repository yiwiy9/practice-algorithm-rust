use itertools::Itertools;
use proconio::input;
use superslice::*;

/// https://atcoder.jp/contests/joisc2011/tasks/joisc2011_ioi
/// https://www2.ioi-jp.org/camp/2011/2011-sp-tasks/2011-sp-day4.pdf
/// https://drken1215.hatenablog.com/entry/2020/12/04/002600
///
/// 金メダルが取れる・取れないの境界を求めて、以上か否かの判定をしようとしたが、
/// それだとうまくいかない
/// => それぞれの人に対して、自分より得点が大きい人が上に L人 以下なら金メダル圏内
///    とすることで、二分探索で解ける
/// => 実装上手くいかなければ着眼点を少しずらす（競プロっぽく各要素ごとにどうかを考えると良さそう）
fn main() {
    input! {
        k: usize,
        n: i64,
        m: i64,
        p: [i64; k],
    }

    let mut p_sorted = p.clone();
    p_sorted.sort();

    let v = 100 * (n - m);
    let l = k.div_ceil(12) - 1; // 自分より上にl人以下ならその人は金メダル圏内

    // x点より多い人が何人いるか（> x）
    // ただし y > x の場合は自分を 1 引く
    let calc = |x: i64, y: i64| -> usize {
        let it = p_sorted.upper_bound(&x); // <=x の人数
        if y > x {
            k - it - 1
        } else {
            k - it
        }
    };

    let mut a = vec![];
    let mut b = vec![];

    for (i, &p_i) in p.iter().enumerate() {
        // A: 確実に金
        if calc(p_i - v, p_i) <= l {
            a.push(i + 1);
        }
        // B: 可能性あり
        if calc(p_i + v, p_i) <= l {
            b.push(i + 1);
        }
    }

    if !a.is_empty() {
        println!("{}", a.iter().join("\n"));
    }
    println!("--------");
    if !b.is_empty() {
        println!("{}", b.iter().join("\n"));
    }
}
