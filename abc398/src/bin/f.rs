use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/abc398/tasks/abc398_f
/// https://atcoder.jp/contests/abc398/editorial/12501
/// まず、 S と S を反転させた文字列をこの順に連結した文字列は明らかに回文なので、正解となる文字列の長さが 2×∣S∣ 以下であることが分かります。
/// したがって、問題は S の末尾から何文字目で折り返すか、その最大値を求めることに帰着します。
/// =>「S の接尾辞であって、最長の回文を求めよ。」
///
/// 文字列アルゴリズムで O(|S|) で求められる。
///
/// 1. Manacher 法を用いる。
/// 文字列が与えられた時、各 i について「文字 i を中心とする最長の回文の半径」を記録した配列 R を O(|S|) で構築するアルゴリズムです。
/// 半径というのは、(全長+1)/2です。
/// ちなみに、普通のManacherをやると奇数長の回文しか検出できませんが、「a$b$a$a$b」みたいにダミー文字を間に挟むと偶数長のものも検出できるようにできます。
/// https://snuke.hatenablog.com/entry/2014/12/02/235837
fn main() {
    input! {
        s: Chars,
    }

    let r = manacher_with_dummy(&s);

    let mut k = 0;
    // 末尾の最大の回文の長さを求めれば良いので、真ん中から見ていく
    for i in s.len()..r.len() {
        if r[i] >= s.len() - k {
            break;
        }
        k += 1;
    }

    let mut ans = s.clone();
    ans.extend(s.iter().take(k).rev());
    println!("{}", ans.iter().collect::<String>());
}

/// 文字列アルゴリズム: Manacher 法
/// 文字列が与えられた時、各 i について「文字 i を中心とする最長の回文の半径」を記録した配列 R を O(|S|) で構築するアルゴリズムです。
/// 半径というのは、(全長+1)/2です。
/// ちなみに、普通の Manacher をやると奇数長の回文しか検出できませんが、「a$b$a$a$b」みたいにダミー文字を間に挟むと偶数長のものも検出できるようにできます。
/// https://snuke.hatenablog.com/entry/2014/12/02/235837
pub fn manacher(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut r = vec![0; n];
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while i >= j && i + j < n && s[i - j] == s[i + j] {
            j += 1;
        }
        r[i] = j;
        let mut k = 1;
        while i >= k && k + r[i - k] < j {
            r[i + k] = r[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    r
}
/// ダミー文字を挿入して偶数長の回文にも対応する Manacher 法
/// 戻り値は、ダミー挿入後の文字列に対する回文半径配列（中心 i に対して r[i]）
pub fn manacher_with_dummy(s: &Vec<char>) -> Vec<usize> {
    let mut t = Vec::with_capacity(s.len() * 2 + 1);
    t.push('$');
    for &c in s {
        t.push(c);
        t.push('$');
    }
    manacher(&t)
}
