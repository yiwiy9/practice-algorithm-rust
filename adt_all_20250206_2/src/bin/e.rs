use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut imos = vec![0; n];
    for (i, &p_i) in p.iter().enumerate() {
        let diff = (p_i + n - i) % n;
        if diff == 0 {
            imos[n - 1] += 1;
            imos[0] += 1;
            imos[2] -= 1;
        } else {
            imos[diff - 1] += 1;
            if diff + 2 < n {
                imos[diff + 2] -= 1;
            }
        }
    }

    let mut ans = 0;
    let mut cur = 0;
    for &imos_i in &imos {
        cur += imos_i;
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
