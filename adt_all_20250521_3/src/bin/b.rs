use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let max_p = *p.iter().max().unwrap();
    let max_p_cnt = p.iter().filter(|&&x| x == max_p).count();

    println!(
        "{}",
        if p[0] < max_p {
            max_p - p[0] + 1
        } else if max_p_cnt == 1 {
            0
        } else {
            1
        }
    );
}
