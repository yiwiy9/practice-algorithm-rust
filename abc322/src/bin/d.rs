use proconio::{input, marker::Chars};

/**
 * https://atcoder.jp/contests/abc322/tasks/abc322_d
 * https://atcoder.jp/contests/abc322/editorial/7302
 * 1番目のポリオミノの向きは固定できる
 */
fn main() {
    input! {
        s: [Chars; 4],
        mut t: [Chars; 4],
        mut u: [Chars; 4],
    }

    if s.iter()
        .map(|row| row.iter().filter(|&&c| c == '#').collect::<Vec<_>>().len())
        .sum::<usize>()
        + t.iter()
            .map(|row| row.iter().filter(|&&c| c == '#').collect::<Vec<_>>().len())
            .sum::<usize>()
        + u.iter()
            .map(|row| row.iter().filter(|&&c| c == '#').collect::<Vec<_>>().len())
            .sum::<usize>()
        != 16
    {
        println!("No");
        return;
    }

    let in_field = |i: i32, j: i32| ((0..4).contains(&i) && (0..4).contains(&j));

    for _ in 0..4 {
        for _ in 0..4 {
            let field = vec![vec![0; 4]; 4];
            for d_s_i in -3..3 {
                for d_s_j in -3..3 {
                    let mut s_field = field.clone();

                    for (s_i, s_row) in s.iter().enumerate() {
                        for (s_j, &c) in s_row.iter().enumerate() {
                            let n_s_i = s_i as i32 + d_s_i;
                            let n_s_j = s_j as i32 + d_s_j;
                            if c == '#' && in_field(n_s_i, n_s_j) {
                                s_field[n_s_i as usize][n_s_j as usize] += 1;
                            }
                        }
                    }

                    for d_t_i in -3..3 {
                        for d_t_j in -3..3 {
                            let mut s_t_field = s_field.clone();

                            for (t_i, t_row) in t.iter().enumerate() {
                                for (t_j, &c) in t_row.iter().enumerate() {
                                    let n_t_i = t_i as i32 + d_t_i;
                                    let n_t_j = t_j as i32 + d_t_j;
                                    if c == '#' && in_field(n_t_i, n_t_j) {
                                        s_t_field[n_t_i as usize][n_t_j as usize] += 1;
                                    }
                                }
                            }

                            for d_u_i in -3..3 {
                                for d_u_j in -3..3 {
                                    let mut s_t_u_field = s_t_field.clone();

                                    for (u_i, u_row) in u.iter().enumerate() {
                                        for (u_j, &c) in u_row.iter().enumerate() {
                                            let n_u_i = u_i as i32 + d_u_i;
                                            let n_u_j = u_j as i32 + d_u_j;
                                            if c == '#' && in_field(n_u_i, n_u_j) {
                                                s_t_u_field[n_u_i as usize][n_u_j as usize] += 1;
                                            }
                                        }
                                    }

                                    if s_t_u_field
                                        .iter()
                                        .all(|row| row.iter().all(|&cnt| cnt == 1))
                                    {
                                        println!("Yes");
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            u = rotate_square(&u);
        }
        t = rotate_square(&t);
    }

    println!("No");
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
