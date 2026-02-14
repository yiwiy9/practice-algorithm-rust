use proconio::input;

/// https://atcoder.jp/contests/joi2023ho/tasks/joi2023ho_b
/// https://www2.ioi-jp.org/joi/2022/2023-ho/2023-ho-t2-review.pdf
///
/// 住人 i への献本により住人 j が本を買う条件
/// |X[i] – X[j]| ≦ E[i] – E[j]
/// X[i] – X[j] ≦ E[i] – E[j] かつ –(X[i] – X[j]) ≦ E[i] – E[j]
/// X[i] – E[i] ≦ X[j] – E[j] かつ X[i] + E[i] ≧ X[j] + E[j]
/// a[i] = X[i] – E[i]
/// b[i] = X[i] + E[i]
/// とすると
/// a[i] ≦ a[j] かつ b[i] ≧ b[j]
fn main() {
    input! {
        n: usize,
        xe: [(i64, i64); n],
    }

    let mut ab: Vec<(i64, i64)> = xe
        .into_iter()
        .map(|(x, e)| (x - e, x + e)) // a, b
        .collect();

    // a 昇順、a が同じなら b 降順（支配判定を素直にするため）
    ab.sort_by(|(a1, b1), (a2, b2)| a1.cmp(a2).then_with(|| b2.cmp(b1)));

    let mut ans = 0usize;
    let mut max_b = i64::MIN;

    for (_a, b) in ab {
        if b > max_b {
            ans += 1;
            max_b = b;
        }
    }

    println!("{}", ans);
}
