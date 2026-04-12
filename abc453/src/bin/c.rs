use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    }

    let mut ans: usize = 0;
    for bit in 0..(1 << n) {
        let mut cur = vec![0; n];
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                cur[i] = l[i];
            } else {
                cur[i] = -l[i];
            }
        }

        let mut cnt = 0;
        let mut pos = 0;
        for &l_i in &cur {
            let next_pos = pos + l_i;
            let next_is_right = next_pos >= 0;
            let cur_is_right = pos >= 0;

            if next_is_right != cur_is_right {
                cnt += 1;
            }

            pos = next_pos;
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
