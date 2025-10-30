use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p_0: usize,
        p: [usize; m],
    }

    let mut ans = 0;
    for c_i in &c {
        let mut cur = p_0;
        for (j, d_j) in d.iter().enumerate() {
            if c_i == d_j {
                cur = p[j];
            }
        }
        ans += cur;
    }

    println!("{}", ans);
}
