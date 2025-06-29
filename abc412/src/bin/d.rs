use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc412/tasks/abc412_d
/// https://atcoder.jp/contests/abc412/editorial/13391
///
/// N<=8で、１つの連結成分には最低3頂点必要なので、連結成分の個数は高々2個
/// → これに気づけず敗北
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut edge_count = vec![vec![0; n]; n];
    for &(a, b) in &ab {
        edge_count[b][a] += 1;
        edge_count[a][b] += 1;
    }

    let mut order = (0..n).collect::<Vec<_>>();
    let mut ans = 1 << 30;

    // next_permutation()
    permutohedron::heap_recursive(&mut order, |order| {
        for bit in 0..(1 << n) {
            let mut first = vec![];
            let mut second = vec![];
            for i in 0..n {
                if (bit >> i) & 1 != 1 {
                    first.push(order[i]);
                } else {
                    second.push(order[i]);
                }
            }

            let first_n = first.len();
            let second_n = second.len();

            if first_n < 3 || (second_n > 0 && second_n < 3) {
                continue;
            }

            let mut cur = m as i32;
            cur += if edge_count[first[0]][first[first_n - 1]] > 0 {
                -1
            } else {
                1
            };
            for i in 0..first_n - 1 {
                cur += if edge_count[first[i]][first[i + 1]] > 0 {
                    -1
                } else {
                    1
                };
            }

            if second_n >= 3 {
                cur += if edge_count[second[0]][second[second_n - 1]] > 0 {
                    -1
                } else {
                    1
                };
                for i in 0..second_n - 1 {
                    cur += if edge_count[second[i]][second[i + 1]] > 0 {
                        -1
                    } else {
                        1
                    };
                }
            }

            ans = ans.min(cur)
        }
    });

    println!("{}", ans);
}
