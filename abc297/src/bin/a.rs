use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        t: [i32; n],
    }

    let mut before_t = -2 * d;
    for t_i in &t {
        if t_i - before_t <= d {
            println!("{}", t_i);
            return;
        }
        before_t = *t_i;
    }

    println!("{}", -1);
}
