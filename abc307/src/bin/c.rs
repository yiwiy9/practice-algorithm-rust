use proconio::{input, marker::Chars};

fn main() {
    input! {
        h_a: usize,
        _w_a: usize,
        a: [Chars; h_a],
        h_b: usize,
        w_b: usize,
        b: [Chars; h_b],
        h_x: usize,
        w_x: usize,
        x: [Chars; h_x],
    }

    let mut a_based_field = [['.'; 50]; 50];

    for (i, a_row) in a.iter().enumerate() {
        for (j, v) in a_row.iter().enumerate() {
            a_based_field[i + 20][j + 20] = *v;
        }
    }

    for b_i_diff in 0..=(50 - h_b) {
        for b_j_diff in 0..=(50 - w_b) {
            let mut field = a_based_field;

            for (b_i, b_row) in b.iter().enumerate() {
                for (b_j, b_v) in b_row.iter().enumerate() {
                    if *b_v == '#' {
                        field[b_i + b_i_diff][b_j + b_j_diff] = '#';
                    }
                }
            }

            for x_i_diff in 0..=(50 - h_x) {
                for x_j_diff in 0..=(50 - w_x) {
                    let mut ok: bool = true;

                    'check: for i in 0..50 {
                        for j in 0..50 {
                            if i < x_i_diff
                                || i - x_i_diff >= h_x
                                || j < x_j_diff
                                || j - x_j_diff >= w_x
                            {
                                ok &= field[i][j] != '#';
                            } else {
                                ok &= field[i][j] == x[i - x_i_diff][j - x_j_diff];
                            }

                            if !ok {
                                break 'check;
                            }
                        }
                    }

                    if ok {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
