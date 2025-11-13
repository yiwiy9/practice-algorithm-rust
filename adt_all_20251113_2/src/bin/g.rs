use itertools::Itertools;
use proconio::{input, marker::Chars};

/// https://atcoder.jp/contests/adt_all_20251113_2/tasks/abc326_d
/// https://atcoder.jp/contests/adt_all_20251113_2/editorial/7540
/// 各行各列に A がちょうど 1 個あるような A の置き方は 5*4*3*2*1=5!=120 通り
fn main() {
    input! {
        n: usize,
        row: Chars,
        col: Chars,
    }

    let head = |s: &Vec<char>| -> char {
        for &c in s {
            if c != '.' {
                return c;
            }
        }
        '.' // 辿りつかない
    };

    for a in (0..n).permutations(n) {
        for b in (0..n).permutations(n) {
            for c in (0..n).permutations(n) {
                // 各配列のindexが行、要素が列に対応 => そこに A が存在する
                if (0..n).any(|i| a[i] == b[i] || b[i] == c[i] || c[i] == a[i]) {
                    continue;
                }

                let mut s = vec![vec!['.'; n]; n];
                for (i, &j) in a.iter().enumerate() {
                    s[i][j] = 'A';
                }
                for (i, &j) in b.iter().enumerate() {
                    s[i][j] = 'B';
                }
                for (i, &j) in c.iter().enumerate() {
                    s[i][j] = 'C';
                }

                let mut ok = true;
                for i in 0..n {
                    if head(&s[i]) != row[i] {
                        ok = false;
                    }
                }
                for j in 0..n {
                    if head(&s.iter().map(|s_i| s_i[j]).collect_vec()) != col[j] {
                        ok = false;
                    }
                }

                if ok {
                    println!("Yes");
                    println!("{}", s.iter().map(|s_i| s_i.iter().join("")).join("\n"));
                    return;
                }
            }
        }
    }

    println!("No");
}
