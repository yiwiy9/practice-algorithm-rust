use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/joisc2007/tasks/joisc2007_anagra
/// https://www2.ioi-jp.org/camp/2007/2007-sp-tasks/2007-sp-day3_23.pdf
/// https://drken1215.hatenablog.com/entry/2020/12/04/020500
/// => S よりも辞書順で小さい文字列の個数を数え上げて、最後に 1 を足せば OK
/// 1. 最初の i-1 文字は S と一致する
/// 2. i 文字目が S よりも小さい
/// 3. それ以降は自由に並び替えて良い
///
/// 同じものを含む順列
/// 1. すべて異なるものとして並べる
/// 2. 同じものの並び替えの影響を排除するためにその分で割る
/// - (p+q+r)! / (p!*q!*r!)
fn main() {
    input! {
        s: Chars,
    }

    let mut t = s.clone();
    t.sort();

    let mut fac = vec![1; s.len() + 1];
    for i in 1..=s.len() {
        fac[i] = fac[i - 1] * i;
    }

    let permutation = |sum: usize, cnt: &Vec<usize>| -> usize {
        let mut res = fac[sum];
        for &c_cnt in cnt {
            res /= fac[c_cnt];
        }
        res
    };

    let mut cnt = vec![0; 26];
    for &c in &s {
        cnt[c as usize - 'A' as usize] += 1;
    }

    let mut ans: usize = 1;
    for (i, &s_i) in s.iter().enumerate() {
        let s_c = s_i as usize - 'A' as usize;
        for c in 0..s_c {
            if cnt[c] == 0 {
                continue;
            }
            cnt[c] -= 1;
            ans += permutation(s.len() - i - 1, &cnt);
            cnt[c] += 1;
        }
        cnt[s_c] -= 1;
    }

    println!("{}", ans);
}
