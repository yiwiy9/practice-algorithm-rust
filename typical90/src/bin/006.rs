use proconio::{input, marker::Chars};

/**
 * https://twitter.com/e869120/status/1379202843622576130/photo/1
 * 辞書順最小は前から貪欲法
 * 1文字目が'b'のものは1文字目が'a'のものに絶対勝てない
 * => 各文字目で同じことが言える
 * => あとは優先的に'a'を選んだ場合に文字数の長さが条件を満たすかを判定
 * => 各文字目における各アルファベットを選ぶことのできる最小のインデックスを配列で保持する
 */
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut s_right_idx = vec![[n; 26]; n + 1];

    for (i, &c) in s.iter().enumerate().rev() {
        s_right_idx[i] = s_right_idx[i + 1];
        let j = c as usize - 'a' as usize;
        s_right_idx[i][j] = i;
    }

    let mut ans = vec![];
    let mut i = 0;
    while i < n && ans.len() < k {
        for j in 0..26 {
            if n - s_right_idx[i][j] >= k - ans.len() {
                ans.push(std::char::from_u32((j + 'a' as usize) as u32).unwrap());
                i = s_right_idx[i][j] + 1;
                break;
            }
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
