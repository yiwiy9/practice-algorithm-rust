use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc310/tasks/abc310_d
 * https://atcoder.jp/contests/abc310/editorial/6783
 * N人をTチームに分ける実装が思いつかなかった（nCtではない）
 * => 「考えうる全てのチーム分けを試して、それが条件を満たすかどうかを判定する」のではなく、
 *     条件を満たす状態を維持しながら選手を追加していく方法を考える
 *
 * 1. 選手を順にチームに追加する解法 O(N×N!) 時間
 *  - 1,2,…,i−1 番目の選手を、互いに相性の悪い選手が同じチームにならないようにたかだか T チームに分けているとします
 *  - ここに i 番目の選手を追加するには、たかだか T チームのいずれかに i 番目の選手を入れるか、
 *   （チーム数が T より少ないとき）新しく i 番目の選手しかいないチームを作るかのどちらかです
 *  - 再帰関数で実装する
 *
 * 2. チームごとにメンバーをまとめて決定する解法 O(3^N×T) 時間
 *  - ビットDPを使う
 *  - dp[S][t] := (S に含まれる選手を、互いに相性の悪い選手を含まない t チームに分ける場合の数)と定めます
 *  - S の（二進法に基づいて非負整数へ変換した値による）昇順に DP テーブルを走査し、
 *    {1,2,…,N}∖S のうち最小の整数に対応する選手が属するチームのメンバーを全探索することで、DP テーブルを埋めることができます
 *  - O(2^N×poly(N)) 時間の前計算を行って、相性の悪い選手の組を含まないチームをすべて列挙しておくことで、計算量を O(3^N×T) 時間とできます
 */
fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    // hate[i] の j ビットめが 1 => i 番目の選手と j 番目の選手の相性が悪い (0-indexed)
    let mut hate = vec![0; n];
    for (a, b) in ab {
        // hate[a] |= 1 << b; 番号の小さい選手からチームに追加していくため不要
        hate[b] |= 1 << a;
    }

    // 解法2

    // possible_team[S] := S からなるチームを作ったとき、相性の悪い選手の組がいない
    // O(2^N N^2/w) 時間
    let mut possible_teams = vec![false; 1 << n];
    for team in 0..(1 << n) {
        let mut hate_members = 0;
        for i in 0..n {
            if team & (1 << i) != 0 {
                hate_members |= hate[i];
            }
        }
        if team & hate_members == 0 {
            possible_teams[team] = true;
        }
    }

    let mut dp = vec![vec![0; t + 1]; 1 << n];
    dp[0][0] = 1;
    for bit in 0..((1 << n) - 1) {
        // 残っているうち番号の一番小さい選手を追加
        let next_min_bit = (bit + 1) | bit;

        // 残っているうち番号の一番小さい選手が属するチームを全探索
        let mut next_bit = next_min_bit;
        while next_bit < (1 << n) {
            // 新たにbitが立つところが追加するチーム(=メンバーの集合)になる
            let new_team = next_bit ^ bit;

            if possible_teams[new_team] {
                for j in 0..t {
                    dp[next_bit][j + 1] += dp[bit][j];
                }
            }

            // 次の選手を追加
            next_bit = (next_bit + 1) | next_min_bit;
        }
    }

    println!("{}", dp[(1 << n) - 1][t]);
}
