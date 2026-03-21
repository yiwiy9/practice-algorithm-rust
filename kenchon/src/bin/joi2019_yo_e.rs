use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/joi2019yo/tasks/joi2019_yo_e
/// https://drken1215.hatenablog.com/entry/2018/12/17/235900
///
/// > とりあえず、区間をどうのこうのする問題は
/// >
/// > - 区間を左から順に処理していく (適切に区間の始点や終点でソートしておく)
/// > - マスを左から順に処理していく
/// >
/// > のどちらにしようか悩むのだけど、
/// > 今回に関してはある区間を選んだとして、その中のどの点を選ぶかで結局各マスに関する考察が必要になってしまう。
/// > ので、マスを左から順に処理する DP にしてみる。
///
/// - `prev[i]`: `i-1` を選ぶ直前に選べる位置の上限
/// - `dp[i]`: `i-1` を最後に選ぶときの最大値
/// - `prefix_max[i]`: `dp[0..=i]` の最大値
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        lr: [(Usize1, Usize1); m],
    }

    // prev[i] は「i-1 を選ぶ直前に選べる位置の上限」
    // 番兵込みで n+1 長さにする
    let mut prev = vec![n; n + 1];
    for i in 1..=n {
        prev[i] = i - 1;
    }

    // 区間 [l, r] があると、r を選ぶ直前には l 以上を選べない
    // 1-index に合わせて prev[r+1] を更新する
    for &(l, r) in &lr {
        prev[r + 1] = prev[r + 1].min(l);
    }

    // 右側の厳しい制約を左へ伝播
    for i in (0..n).rev() {
        prev[i] = prev[i].min(prev[i + 1]);
    }

    let mut dp = vec![0usize; n + 1];
    let mut prefix_max = vec![0usize; n + 1];

    for i in 1..=n {
        dp[i] = prefix_max[prev[i]] + a[i - 1];
        prefix_max[i] = prefix_max[i - 1].max(dp[i]);
    }

    println!("{}", prefix_max[n]);
}
