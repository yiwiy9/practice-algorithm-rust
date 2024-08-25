use itertools::Itertools;
use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc368/tasks/abc368_e
 * https://atcoder.jp/contests/abc368/editorial/10752
 * 前の電車の到着が１本でも遅れていれば、その遅れに合わせて次の電車の発車も遅らせる必要がある。
 * ここで注意するべきは、前の電車として考慮する必要があるのは、時刻表上の到着時刻が次の電車の発車時刻よりも前の電車である。
 * つまり、1=>2 の電車でも、S1 > T2 であれば、考慮する必要ない。（単純なダイクストラがダメな理由）
 *
 * イベントソート:　黄diff
 * => 発車時刻・到着時刻を同列のイベントとして時刻でソートする
 *
 * 1. 電車 i が発車する際には、現在までに処理した電車（すなわち時刻表上の到着時刻が
 *    電車 i の時刻表上の発車時刻までの電車）の駅 Ai への実際の到着時刻に基づいて Xi を計算する
 * 2. 電車 i が到着する際には、Xi に基づいて、駅 Bi に実際に到着した到着時刻を記録する
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        x_1: i64,
        abst: [(Usize1,Usize1,i64,i64); m],
    }

    let mut event = vec![];
    for (i, &(a, b, s, t)) in abst.iter().enumerate() {
        // 発車時刻と到着時刻が同じ場合は、到着時刻を先に処理する（発車時刻に影響するため）
        event.push((s, 1, a, i));
        event.push((t, 0, b, i));
    }
    event.sort();

    let mut ans = vec![0; m];
    ans[0] = x_1;

    let mut last_arrived = vec![0; n];
    for (time, is_start, station, i) in event {
        if is_start == 1 {
            if i == 0 {
                // 最初の電車は処理済み
                continue;
            }
            ans[i] = (last_arrived[station] - time).max(0);
        } else {
            last_arrived[station] = last_arrived[station].max(time + ans[i]);
        }
    }

    println!("{}", ans.iter().skip(1).join(" "));
}
