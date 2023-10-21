use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * https://atcoder.jp/contests/abc325/tasks/abc325_d
 * https://atcoder.jp/contests/abc325/editorial/7448
 *
 * 単純な区間スケジュール問題ではないが、以下の貪欲が成り立つ
 * => t=1,2,3,…, の順に、時刻tに印字機の範囲内にある商品であって、まだ印字されていないものがあるならば、そのうち印字機の範囲から出るのが最も早い商品に印字する。
 *
 * しかし、tは最大で10^18程度になりうるため、forでは間に合わない
 * => ある時刻tにおいて、印字できる商品が１つもなかった場合、t 以降で商品が印字機の範囲内に新たに入ってくる最初の時刻をt′として、t←t′と更新する。
 *
 * 印字できる商品の中から印字機の範囲を出る時刻が最も早い商品を選ぶのは優先度付きキュー
 * （印字できる商品がない場合に）t′を求めるのは尺取り法
 *
 */
fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }

    let mut interval = vec![];
    for &(t, d) in &td {
        interval.push((t, t + d));
    }
    interval.sort();

    let mut heap = BinaryHeap::new();
    let mut t = 0;
    let mut i = 0;
    let mut ans = 0;

    loop {
        // 印字できる商品がない場合
        if heap.is_empty() {
            if i == n {
                break;
            }
            t = interval[i].0;
        }

        // 時刻tから新たに印字できるようになる商品の追加
        while i < n && interval[i].0 == t {
            heap.push(Reverse(interval[i].1));
            i += 1;
        }

        // 時刻t時点で印字されずに印字機の範囲から出た商品を削除
        while !heap.is_empty() && heap.peek().unwrap().0 < t {
            heap.pop();
        }

        // 印字機の範囲から出るのが最も早い商品に印字する
        if !heap.is_empty() {
            heap.pop();
            ans += 1;
        }

        // 時間を進める
        t += 1;
    }

    println!("{}", ans);
}
