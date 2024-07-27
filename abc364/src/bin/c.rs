use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));

    let mut a_cnt = 0;
    let mut a_sum = 0;
    for &a_i in &a {
        if a_sum > x {
            break;
        }
        a_sum += a_i;
        a_cnt += 1;
    }

    let mut b_cnt = 0;
    let mut b_sum = 0;
    for &b_i in &b {
        if b_sum > y {
            break;
        }
        b_sum += b_i;
        b_cnt += 1;
    }

    println!("{}", a_cnt.min(b_cnt));
}
