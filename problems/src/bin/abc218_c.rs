use proconio::{input, marker::Chars};

/**
 * C - Shapes
 * https://atcoder.jp/contests/abc218/tasks/abc218_c
 * https://atcoder.jp/contests/abc218/editorial/2598
 *
 * S と T に含まれる # の個数のチェックを先に行うことで、S の全ての # が T の # と重なるか確認すれば良い
 * => Tの全探索は不要
 *
 * 両者が一致するためには、S の最も左上のマスと T の最も左上のマスが一致することが必要であり、そのようなマスを求めることで平行移動量は一意に決まる
 * => find_left_top
 *
 * S に対して 90 度回転を何回行うか 4 通りを全探索
 * => rotate_square
 */
fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    }

    if s.iter()
        .map(|row| row.iter().filter(|&&c| c == '#').collect::<Vec<_>>().len())
        .sum::<usize>()
        != t.iter()
            .map(|row| row.iter().filter(|&&c| c == '#').collect::<Vec<_>>().len())
            .sum::<usize>()
    {
        println!("No");
        return;
    }

    let (t_left_top_i, t_left_top_j) = find_left_top(&t).unwrap();

    for _ in 0..4 {
        let mut ok = true;

        let (s_left_top_i, s_left_top_j) = find_left_top(&s).unwrap();
        let offset_i = t_left_top_i as i32 - s_left_top_i as i32;
        let offset_j = t_left_top_j as i32 - s_left_top_j as i32;

        'check: for (s_i, s_row) in s.iter().enumerate() {
            for (s_j, &s_ij) in s_row.iter().enumerate() {
                let t_i = s_i as i32 + offset_i;
                let t_j = s_j as i32 + offset_j;

                if t_i < 0 || n as i32 <= t_i || t_j < 0 || n as i32 <= t_j {
                    if s_ij == '#' {
                        ok = false;
                        break 'check;
                    }
                    continue;
                }

                let t_i = t_i as usize;
                let t_j = t_j as usize;
                if s_ij != t[t_i][t_j] {
                    ok = false;
                    break 'check;
                }
            }
        }

        if ok {
            println!("Yes");
            return;
        }

        s = rotate_square(&s);
    }

    println!("No");
}

fn find_left_top(field: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, row) in field.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                return Some((i, j));
            }
        }
    }
    None
}

fn rotate_square(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; field.len()]; field.len()];

    for (i, row) in field.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            result[j][field.len() - 1 - i] = c;
        }
    }

    result
}
