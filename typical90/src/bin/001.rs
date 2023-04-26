use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.push(l);

    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut prev_cut = 0;
        let mut cut_cnt = 0;
        for a_i in &a {
            if a_i - prev_cut >= mid {
                prev_cut = *a_i;
                cut_cnt += 1;
            }
        }

        if cut_cnt > k {
            left = mid
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
