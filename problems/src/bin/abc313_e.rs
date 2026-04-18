use ac_library::modint::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc313/tasks/abc313_e
/// - 何ごとの話にすると見やすい？ 何だけ持てばいい？
///   右側の suffix が 1 文字になるまでに必要な操作回数の話にする
/// - それについて、何が分かれば答えになる？
///   各 suffix の操作回数が右から順に分かれば、全体の操作回数が答えになる
/// - 何を捨ててよく、なぜそれで足りる？ 何が効く / 何が禁止？
///   文字列全体の形は持たなくてよい
///   2以上の数字が隣り合うとその構造が消えず無限なので -1
///   そうでなければ、左への影響は「右側の操作回数」と「今の数字」だけで決まる
///   更新は足し算と掛け算だけなので、途中から mod で持ってよい
/// - その情報をどう更新 / 判定 / 集計すれば実装できる？
///   まず隣接する 2 文字が両方 2 以上なら -1
///   そうでなければ右から見て、今の suffix の操作回数を cur とすると
///   各桁 a[i] について cur = (cur + 1) * a[i] を mod で更新していく
///
/// 解説AC
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let a = s
        .iter()
        .map(|&s_i| s_i as usize - '0' as usize)
        .collect_vec();

    let mut before = a[0];
    for &a_i in a.iter().skip(1) {
        if before > 1 && a_i > 1 {
            println!("-1");
            return;
        }
        before = a_i;
    }

    let mut ans = Mint::new(0);
    for &a_i in a.iter().skip(1).rev() {
        ans = (ans + 1) * a_i;
    }

    println!("{}", ans.val());
}
