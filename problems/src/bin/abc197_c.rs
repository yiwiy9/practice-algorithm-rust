use proconio::input;
const INF: usize = 1 << 30;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = INF;
    for bit in 0..(1 << (n - 1)) {
        let mut cur_or = 0;
        let mut cur_xor = 0;

        for i in 0..n {
            cur_or |= a[i];
            if bit & (1 << i) != 0 {
                if cur_xor == 0 {
                    cur_xor = cur_or;
                } else {
                    cur_xor ^= cur_or;
                }
                cur_or = 0;
            }
        }
        if cur_xor == 0 {
            cur_xor = cur_or;
        } else {
            cur_xor ^= cur_or;
        }
        ans = ans.min(cur_xor);
    }

    println!("{}", ans);
}
