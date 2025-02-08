use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n],
    }

    let mut cnt = 0;
    for &h_i in &h {
        if h_i <= m {
            cnt += 1;
            m -= h_i;
        } else {
            break;
        }
    }

    println!("{}", cnt);
}
