use proconio::input;

/**
 * https://atcoder.jp/contests/abc244/tasks/abc244_d
 * https://atcoder.jp/contests/abc244/editorial/3594
 * 解説がほーって感じ。
 *
 * さて、3 要素の並べ替えではなく、n 要素であった場合はどうすれば良いでしょうか。ここで登場するのが、転倒数 です。
 *
 * 転倒数は、(1,2,…,n) の並べ替え (A1,A2,…,An) に対して、 i<j かつ Ai >Aj であるような整数の組 (i,j) の数として定義されます。
 * そして、転倒数が偶数であるような並べ替えを 偶置換 、転倒数が奇数であるような並べ替えを 奇置換 と呼びます。
 * これが先ほどの 2 つのグループと対応します。偶置換に 1 回の操作を行うと必ず奇置換に、奇置換に 1 回の操作を行うと必ず偶置換になるので、
 * どんなに大きい偶数回の操作を行なっても、違うグループに変換することはできないのです。
 */
fn main() {
    input! {
        s: [char; 3],
        t: [char; 3],
    }

    let diff_cnt = (0..3).filter(|&i| s[i] != t[i]).count();

    println!("{}", if diff_cnt == 2 { "No" } else { "Yes" });
}
