use proconio::input;

/**
 * https://atcoder.jp/contests/abc202/tasks/abc202_d
 * https://atcoder.jp/contests/abc202/editorial/1860
 * 辞書順が関係する問題では、前の文字から決めていくというのが定石
 * => 先頭が a であるような文字列の総数が K 個以上なら a。そうでないなら b
 *
 * a が i 個と b が j 個からなる文字列の総数 C(i,j) を前もって計算する
 * => 1桁目から a or b を選択する単純なDP
 *
 * あとは再帰的に1桁目から求めていく
 */
fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    let mut dp = vec![vec![0; b + 1]; a + 1];
    dp[0][0] = 1;
    for i in 0..=a {
        for j in 0..=b {
            if i > 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }

    println!("{}", rec(&dp, a, b, k));
}

fn rec(dp: &Vec<Vec<usize>>, a: usize, b: usize, k: usize) -> String {
    if a == 0 {
        return String::from("b").repeat(b);
    }
    if b == 0 {
        return String::from("a").repeat(a);
    }

    if k <= dp[a - 1][b] {
        // 先頭が a であるような文字列の総数が K 個以上なら a
        String::from("a") + &rec(dp, a - 1, b, k)
    } else {
        // そうでないなら b
        String::from("b") + &rec(dp, a, b - 1, k - dp[a - 1][b])
    }
}
