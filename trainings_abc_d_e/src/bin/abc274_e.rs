use proconio::input;

const INF: f64 = (1_usize << 60) as f64;

/**
 * ビットDP(bit DP)
 * https://atcoder.jp/contests/abc274/tasks/abc274_e
 * https://atcoder.jp/contests/abc274/editorial/5020
 *
 * dp[i][b] := 現在地が街 i で、b のうち立っているビットが訪問済みの街であるときの所要時間の最小値
 * => 最初に原点から各街への移動を済ませた状態でDPをスタートする
 * => あとは b の昇順に任せて、緩和処理を行えば良い
 * => 最後に再び各街から原点への移動を計算して、最小値を答えとして出力する
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(f64, f64); n+m],
    }

    let dist = |p: (f64, f64), q: (f64, f64)| -> f64 {
        ((p.0 - q.0) * (p.0 - q.0) + (p.1 - q.1) * (p.1 - q.1)).sqrt()
    };

    let inv_speed = |bit: usize| -> f64 {
        let boost_bit = bit >> n;
        let boost_chars = decimal_to_chars(boost_bit, 2);
        let boost_cnt = boost_chars.iter().filter(|&&c| c == '1').count();

        let mut res = 1.0;
        for _ in 0..boost_cnt {
            res /= 2.0;
        }
        res
    };

    let mut dp = vec![vec![INF; 1 << (n + m)]; n + m];
    for (i, &xy) in xy.iter().enumerate() {
        dp[i][1 << i] = dist((0.0, 0.0), xy);
    }

    for bit in 1..(1 << (n + m)) {
        let inv_speed = inv_speed(bit);
        for from in 0..n + m {
            if bit & (1 << from) == 0 {
                continue;
            }

            for to in 0..n + m {
                if bit & (1 << to) != 0 {
                    continue;
                }

                let next_bit = bit | (1 << to);
                let new_time = dp[from][bit] + dist(xy[from], xy[to]) * inv_speed;
                dp[to][next_bit] = dp[to][next_bit].min(new_time);
            }
        }
    }

    let mut ans = INF;
    for i in 0..n + m {
        let mut bit = (1 << n) - 1; // 街のみ全て訪れた状態
        while bit < (1 << (n + m)) {
            ans = ans.min(dp[i][bit] + dist(xy[i], (0.0, 0.0)) * inv_speed(bit));
            bit += 1 << n; // 取る宝箱の種類を全探索する
        }
    }

    println!("{:.10}", ans);
}

pub fn decimal_to_chars(mut n: usize, base: usize) -> Vec<char> {
    if n == 0 {
        return vec!['0'];
    }
    let mut result = Vec::new();
    while n > 0 {
        let char = std::char::from_digit((n % base) as u32, base as u32).unwrap();
        result.push(char);
        n /= base;
    }
    result.iter().rev().copied().collect()
}
