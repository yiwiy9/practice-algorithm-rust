use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut same_pos = 0;
    let mut diff_pos = 0;
    for (i, &a_i) in a.iter().enumerate() {
        for (j, &b_j) in b.iter().enumerate() {
            if a_i != b_j {
                continue;
            }
            if i == j {
                same_pos += 1;
            } else {
                diff_pos += 1;
            }
        }
    }

    println!("{}", same_pos);
    println!("{}", diff_pos);
}
