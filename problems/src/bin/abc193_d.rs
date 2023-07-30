use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    let mut s_cnt = vec![0; 10];
    let mut t_cnt = vec![0; 10];
    for &c in &s {
        if c == '#' {
            continue;
        }
        s_cnt[c as usize - '0' as usize] += 1;
    }
    for &c in &t {
        if c == '#' {
            continue;
        }
        t_cnt[c as usize - '0' as usize] += 1;
    }

    let denominator = 9.0 * k as f64 - 8.0;
    let mut ans = 0.0;
    for i in 1..10 {
        for j in 1..10 {
            if s_cnt[i] + t_cnt[i] >= k || s_cnt[j] + t_cnt[j] >= k {
                continue;
            }

            let s_points = calc_points(&s_cnt, i);
            let t_points = calc_points(&t_cnt, j);
            if s_points <= t_points {
                continue;
            }

            let mut numerator = k - s_cnt[i] - t_cnt[i];
            if i == j {
                numerator *= k - s_cnt[j] - t_cnt[j] - 1;
            } else {
                numerator *= k - s_cnt[j] - t_cnt[j];
            }

            ans += numerator as f64 / (denominator * (denominator - 1.0));
        }
    }

    println!("{:.8}", ans);
}

fn calc_points(cnt: &[usize], i: usize) -> usize {
    let mut res = 0;
    for (j, &cnt_j) in cnt.iter().enumerate() {
        let mut c = cnt_j;
        if j == i {
            c += 1;
        }

        let mut p = j;
        for _ in 0..c {
            p *= 10;
        }
        res += p;
    }
    res
}
