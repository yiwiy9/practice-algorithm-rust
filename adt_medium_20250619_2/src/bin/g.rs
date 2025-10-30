use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        mut td: [(i64, i64); n], // (t, d) は広めに i64 推奨
    }

    // 到着時刻 t 昇順に並べる
    td.sort_unstable();

    let mut ans: i64 = 0;
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // 締切(= t+d)の最小ヒープ
    let mut i: usize = 0; // 未投入データの先頭
    let mut cur: i64 = 0; // 現在時刻

    // まだ到着が残っている or 処理待ちがある間は続ける
    while i < n || !heap.is_empty() {
        // ヒープが空なら次の到着時刻までジャンプ
        if heap.is_empty() {
            cur = td[i].0;
        }

        // 現在時刻までに到着したものをすべて投入（締切だけ入れる）
        while i < n && td[i].0 <= cur {
            let (t, d) = td[i];
            heap.push(Reverse(t + d));
            i += 1;
        }

        // 期限切れを除去
        while let Some(&Reverse(dead)) = heap.peek() {
            if dead < cur {
                heap.pop();
            } else {
                break;
            }
        }

        // 1つ処理して時刻を進める
        if !heap.is_empty() {
            heap.pop();
            ans += 1;
            cur += 1;
        }
    }

    println!("{}", ans);
}
