use itertools::Itertools;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc221/tasks/abc221_d
 * https://atcoder.jp/contests/abc221/editorial/2722
 * 全ての日を配列にしなくとも、ログインした日とログインしなくなった日でカウンターを増減させればimos法っぽくできる
 */
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }
    let mut event_days = vec![];
    ab.iter().for_each(|&(a, b)| {
        event_days.push((a, 1)); // ログインした日 +1人
        event_days.push((a + b, -1)); // ログインしなくなった日 -1人
    });
    event_days.sort();

    let mut cnt = 0;
    let mut ans = vec![0; n + 1];
    for (i, &(day, event)) in event_days.iter().enumerate() {
        if i + 1 < event_days.len() {
            cnt += event;
            ans[cnt as usize] += event_days[i + 1].0 - day;
        }
    }

    println!("{}", ans.iter().skip(1).join(" "));
}
