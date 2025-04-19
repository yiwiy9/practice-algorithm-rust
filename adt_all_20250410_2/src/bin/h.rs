use proconio::input;

/// dp[b] := bのうち立っているビットのカードが場に残っている状態で先手が勝つことができる場合はtrue、そうでない場合はfalse
fn main() {
    input! {
        n: usize,
        ab: [(usize,usize); n],
    }

    let mut dp = vec![false; 1 << n];
    for bit in 1..(1 << n) {
        let mut can_win = false;

        // 次の状態で先手が負けるものが一つでもあれば、その状態になるように先手が選べば勝つことができる
        for i in 0..n {
            if bit & (1 << i) == 0 {
                continue;
            }

            for j in i + 1..n {
                if bit & (1 << j) == 0 {
                    continue;
                }

                if ab[i].0 != ab[j].0 && ab[i].1 != ab[j].1 {
                    continue;
                }

                // i, j のカードを選ぶ
                let mut next_bit = bit;
                next_bit ^= 1 << i;
                next_bit ^= 1 << j;

                // 次の状態で先手が負ける場合
                if !dp[next_bit] {
                    can_win = true;
                    break;
                }
            }
        }

        dp[bit] = can_win;
    }

    println!(
        "{}",
        if dp[(1 << n) - 1] {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
