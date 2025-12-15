use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        field: [[usize; c]; r],
    }

    let mut ans = 0;
    for bit in 0..(1 << r) {
        let mut zero_cnt = vec![0; c];
        for i in 0..r {
            for (j, &num) in field[i].iter().enumerate() {
                if (bit >> i) & 1 == 1 {
                    if num == 1 {
                        zero_cnt[j] += 1;
                    }
                } else {
                    if num == 0 {
                        zero_cnt[j] += 1;
                    }
                }
            }
        }
        ans = ans.max(zero_cnt.iter().map(|&cnt| cnt.max(r - cnt)).sum());
    }

    println!("{}", ans);
}
