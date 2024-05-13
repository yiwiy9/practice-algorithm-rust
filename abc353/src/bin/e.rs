use proconio::{input, marker::Chars};

/**
 * https://atcoder.jp/contests/abc353/tasks/abc353_e
 * https://atcoder.jp/contests/abc353/editorial/9956
 *
 * 文字列を並び替えても結果は変わらないので、文字列をソートしておく。
 * => Ai = f(Si,Si+1) とし、全ての Ai を求める。
 * => ソートされているので、f(Si,Sj) = min(f(Si,Si+1), f(Si+1,Si+2),…, f(Sj-1, Sj)) である。
 * => もとの問題は「(A1,…,AN−1) の全ての非空連続部分列についてminを求め、その和を求めよ」という問題になる。
 * => これは、ヒストグラムの最大長方形問題と同様に、スタックを用いて増加列を管理することで高速に計算できる。
 *    (https://algo-method.com/tasks/945/editorial)
 *
 * 非空連続部分列とは、
 * 配列や文字列の中で、元の配列や文字列の順序を保ちつつ、連続している要素から構成される部分列のことを指します。
 * そして、「非空」とはその部分列が空でない、つまり一つ以上の要素を含んでいることを意味しています。
 */
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    s.sort();

    let mut a = vec![0; n - 1];
    for i in 0..n - 1 {
        let mut j = 0;
        while j < s[i].len() && j < s[i + 1].len() && s[i][j] == s[i + 1][j] {
            j += 1;
        }
        a[i] = j as i64;
    }

    // aの全ての非空連続部分列についてのminの和を求める
    let mut ans = 0;
    let mut stack: Vec<(i64, usize)> = vec![];
    for i in 0..n - 1 {
        // a[i]がminとなる範囲の左端を求める
        let mut left = i;
        while let Some(&top) = stack.last() {
            if top.0 < a[i] {
                break;
            }
            left = top.1;
            stack.pop();
        }
        stack.push((a[i], left));

        // 計算の都合上、番兵を追加
        stack.push((-1, i + 1));
        for j in 0..stack.len() - 1 {
            ans += stack[j].0 * (stack[j + 1].1 - stack[j].1) as i64;
        }
        stack.pop();
    }

    println!("{}", ans);
}
