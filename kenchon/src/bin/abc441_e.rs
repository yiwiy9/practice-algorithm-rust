use ac_library::FenwickTree;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cum = vec![n; n + 1];
    for i in 0..n {
        cum[i + 1] = if s[i] == 'A' {
            cum[i] + 1
        } else if s[i] == 'B' {
            cum[i] - 1
        } else {
            cum[i]
        };
    }

    // 転倒数を考えるときは「今見ている要素を区間の右端にする」と実装がめちゃくちゃシンプルになる
    // 理由: 左端とすると、処理を終えた後に以降のループへの影響を排除するために、副作用を加える必要があり、そこの実装がバグりがち
    let mut ft = FenwickTree::new(2 * n, 0);
    let mut ans: usize = 0;
    for &cum_i in &cum {
        // 自分より左側の文字列の情報が格納されている
        ans += ft.sum(0..cum_i); // その内、自分より累積が小さい範囲の数
        ft.add(cum_i, 1);
    }

    println!("{}", ans);
}
