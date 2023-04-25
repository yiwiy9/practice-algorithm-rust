use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        b: [[i32; n]; n],
    }

    let mut ok_array = [true, true, true, true];
    for (i, row) in a.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            if *num == 0 {
                continue;
            }

            let mut i_mut = i;
            let mut j_mut = j;
            for ok in ok_array.iter_mut() {
                *ok &= b[i_mut][j_mut] == 1;

                let tmp_i = i_mut;
                i_mut = n - 1 - j_mut;
                j_mut = tmp_i;
            }
        }
    }

    println!(
        "{}",
        if ok_array.iter().filter(|&&ok| ok).count() > 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
