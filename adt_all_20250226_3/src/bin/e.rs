use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut t = 0;
    for &h_i in &h {
        let mut h_mut = h_i;
        let div = h_mut / 5;

        t += div * 3;
        h_mut -= div * 5;

        while h_mut > 0 {
            t += 1;
            if t % 3 == 0 {
                h_mut -= 3;
            } else {
                h_mut -= 1;
            }
        }
    }

    println!("{}", t);
}
