use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let mut max_r = 0;
    let mut max_i = 0;
    for (i, &c_i) in c.iter().enumerate() {
        if c_i == t && r[i] > max_r {
            max_r = r[i];
            max_i = i;
        }
    }

    if max_r == 0 {
        t = c[0];
        for (i, &c_i) in c.iter().enumerate() {
            if c_i == t && r[i] > max_r {
                max_r = r[i];
                max_i = i;
            }
        }
    }

    println!("{}", max_i + 1);
}
