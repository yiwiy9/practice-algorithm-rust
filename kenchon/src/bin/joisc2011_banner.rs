use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let update_counter_per_row = |row_i: usize, counter: &mut Vec<Vec<[[i64; 3]; 3]>>, num: i64| {
        for (j, &color) in a[row_i].iter().enumerate() {
            for k in j + 1..w {
                counter[j][k][color][a[row_i][k]] += num;
            }
        }
    };

    let mut counter: Vec<Vec<[[i64; 3]; 3]>> = vec![vec![[[0; 3]; 3]; w]; w];
    for i in 0..h {
        update_counter_per_row(i, &mut counter, 1);
    }

    let mut ans: i64 = 0;
    for (i, row) in a.iter().enumerate() {
        update_counter_per_row(i, &mut counter, -1);
        for (j, &color) in row.iter().enumerate() {
            for k in j + 1..w {
                for j_color in 0..3 {
                    for k_color in 0..3 {
                        let bit = (1 << color) | (1 << row[k]) | (1 << j_color) | (1 << k_color);
                        if bit == 0b111 {
                            ans += counter[j][k][j_color][k_color];
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
