use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        t: usize,
        m: usize,
    }

    let mut memo = HashMap::new();

    for _ in 0..t {
        input! {
            n: usize,
            mut c: [usize; n],
        }

        let mut ans = 1;
        let mut can_use = c.iter().sum::<usize>();
        c.sort_by(|a, b| b.cmp(a));
        for &x in &c {
            ans *= comb(can_use, x, m, &mut memo);
            ans %= m;
            can_use -= x;
        }
        println!("{}", ans);
    }
}

fn comb(n: usize, k: usize, m: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if k == 0 {
        return 1;
    }
    if n == 0 {
        return 0;
    }
    if let Some(&v) = memo.get(&(n, k)) {
        return v;
    }

    if k > n / 2 {
        return comb(n, n - k, m, memo);
    }

    let val = (comb(n - 1, k - 1, m, memo) + comb(n - 1, k, m, memo)) % m;
    memo.insert((n, k), val);
    memo[&(n, k)]
}
