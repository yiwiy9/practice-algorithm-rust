use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    for bit_i in 0..(1 << h1) {
        let mut a_rows = vec![];
        for a_i in 0..h1 {
            if bit_i & (1 << a_i) != 0 {
                a_rows.push(a_i);
            }
        }
        if a_rows.len() != h2 {
            continue;
        }

        for bit_j in 0..(1 << w1) {
            let mut a_columns = vec![];
            for a_j in 0..w1 {
                if bit_j & (1 << a_j) != 0 {
                    a_columns.push(a_j);
                }
            }
            if a_columns.len() != w2 {
                continue;
            }

            let mut ok = true;
            for (i, b_rows) in b.iter().enumerate() {
                for (j, b_ij) in b_rows.iter().enumerate() {
                    if a[a_rows[i]][a_columns[j]] != *b_ij {
                        ok = false;
                    }
                }
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
