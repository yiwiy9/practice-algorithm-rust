use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    }

    s.sort();

    // Ai = f(Si,Si+1)
    // 隣同士の文字列の共通接頭辞の長さを求める
    // => ヒストグラムの最大長方形問題 と同様の方法で解く
    let mut a = vec![0; n - 1];
    for i in 0..n - 1 {
        let mut j = 0;
        while j < s[i].len() && j < s[i + 1].len() && s[i][j] == s[i + 1][j] {
            j += 1;
        }
        a[i] = j as i64;
    }

    // ヒストグラムの最大長方形問題
    // 最長共通接頭辞の長さをスタックで管理し、
    // iごとに今まで見てきたj(<i)を加算する
    // その際に最長共通接頭辞の長さを常に更新していく
    let mut ans = 0;
    let mut stack: Vec<(i64, usize)> = vec![];
    for i in 0..n - 1 {
        // iが一致しているところまでstackで戻っていく
        // => a[i]がminとなる範囲の左端を求める
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
            // 段々になっている長方形を足していく
            // ここで足しあげられるのは、0~iまでの長方形
            ans += stack[j].0 * (stack[j + 1].1 - stack[j].1) as i64;
        }
        stack.pop();
    }

    println!("{}", ans);
}
