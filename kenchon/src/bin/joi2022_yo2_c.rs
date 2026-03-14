use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let mut a_cols = vec![0; w];
    for row in &a {
        for (j, a_ij) in row.iter().enumerate() {
            a_cols[j] += a_ij;
        }
    }
    let a_cols_sum = a_cols.iter().sum::<usize>();

    let mut ans: usize = 0;
    for div_col in 1..=w {
        if a_cols_sum % div_col != 0 {
            continue;
        }
        let a_cols_target = a_cols_sum / div_col;

        let mut ok = true;
        let mut div_j = vec![];
        let mut cur = 0;
        for (j, &a_cols_j) in a_cols.iter().enumerate() {
            cur += a_cols_j;
            if cur > a_cols_target {
                ok = false;
                break;
            } else if cur == a_cols_target {
                cur = 0;
                div_j.push(j);
            }
        }
        if !ok || cur != 0 {
            continue;
        }

        for div_row in 1..=h {
            if a_cols_target % div_row != 0 {
                continue;
            }
            let target = a_cols_target / div_row;

            let mut ok = true;
            let mut cur = vec![0; div_j.len()];
            for i in 0..h {
                let mut j = 0;
                for (cur_j, &cur_div_j) in div_j.iter().enumerate() {
                    while j <= cur_div_j {
                        cur[cur_j] += a[i][j];
                        j += 1;
                    }
                }

                let cur_max = *cur.iter().max().unwrap();
                if cur_max > target {
                    ok = false;
                    break;
                } else if cur_max == target {
                    if cur.iter().all(|&v| v == cur_max) {
                        cur = vec![0; div_j.len()];
                    } else {
                        ok = false;
                        break;
                    }
                }
            }

            if !ok || cur.iter().any(|&v| v != 0) {
                continue;
            }

            ans += 1;
        }
    }

    println!("{}", ans - 1);
}
