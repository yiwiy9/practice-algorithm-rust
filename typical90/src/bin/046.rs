use proconio::input;

const NUM_46: usize = 46;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut a_46 = vec![0_u64; NUM_46];
    let mut b_46 = vec![0_u64; NUM_46];
    let mut c_46 = vec![0_u64; NUM_46];

    for i in 0..n {
        a_46[a[i] % NUM_46] += 1;
        b_46[b[i] % NUM_46] += 1;
        c_46[c[i] % NUM_46] += 1;
    }

    let mut ans: u64 = 0;
    for (a_num, a_cnt) in a_46.iter().enumerate() {
        for (b_num, b_cnt) in b_46.iter().enumerate() {
            for (c_num, c_cnt) in c_46.iter().enumerate() {
                if (a_num + b_num + c_num) % NUM_46 == 0 {
                    ans += a_cnt * b_cnt * c_cnt;
                }
            }
        }
    }

    println!("{}", ans);
}
