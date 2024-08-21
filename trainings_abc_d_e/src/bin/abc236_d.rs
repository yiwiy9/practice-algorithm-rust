use proconio::input;

/**
 * https://atcoder.jp/contests/abc236/tasks/abc236_d
 * https://atcoder.jp/contests/abc236/editorial/3285
 *
 * 前回と同じミス
 * => i, j は２重ループである必要がない
 *    i を決めて、そのiに対してjで全探索すればよい
 */
fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 0..(2 * n - 1) {
        let mut row = vec![0; 2 * n];
        for row_j in row.iter_mut().skip(i + 1) {
            input! {
                a_ij: usize,
            }
            *row_j = a_ij;
        }
        a.push(row);
    }

    println!("{}", rec(&a, n, &mut vec![false; 2 * n], 0));
}

fn rec(a: &Vec<Vec<usize>>, n: usize, used: &mut Vec<bool>, cur: usize) -> usize {
    if used.iter().all(|&u| u) {
        return cur;
    }

    let mut res = 0;

    let mut l = 0;
    for i in 0..2 * n - 1 {
        if !used[i] {
            l = i;
            break;
        }
    }

    used[l] = true;
    for j in l + 1..2 * n {
        if used[j] {
            continue;
        }

        used[j] = true;
        res = res.max(rec(a, n, used, cur ^ a[l][j]));
        used[j] = false;
    }
    used[l] = false;

    res
}
