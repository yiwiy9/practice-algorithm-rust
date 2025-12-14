use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc436/tasks/abc436_f
/// https://atcoder.jp/contests/abc436/editorial/14750
/// こういう区間の問題はとにかく何を固定するかを考える
/// => 区間を固定しようとして失敗しがちで、点を固定すると良いことが多い
///
/// 写っている星の集合の中で最も暗い星が i 番目の星であるような集合を考えると、
/// (左側の明るい星の個数 + 1) × (右側の明るい星の個数 + 1) でその場合の数が求まる
/// => ただの転倒数を求める問題になる
fn main() {
    input! {
        n: usize,
        b: [Usize1; n],
    }

    let mut ft = FenwickTree::new(n, 0);
    let mut ans = 0;
    for &b_i in &b {
        // 自分より左側の星の情報が格納されている
        let left = ft.sum(0..b_i); // その内、自分より明るい星の数
        let right = b_i - left; // 残りの自分より明るい星の数 = 右側の明るい星の個数

        ans += (left + 1) * (right + 1);
        ft.add(b_i, 1);
    }

    println!("{}", ans);
}
