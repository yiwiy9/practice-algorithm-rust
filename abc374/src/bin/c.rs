use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [usize; n],
    }

    let mut ans = 1 << 60;
    for bit in 0..(1 << n) {
        let mut a_sum = 0;
        let mut b_sum = 0;
        for i in 0..n {
            if bit & (1 << i) == 0 {
                a_sum += k[i];
            } else {
                b_sum += k[i];
            }
        }
        let cur = a_sum.max(b_sum);
        ans = ans.min(cur);
    }

    println!("{}", ans);
}
