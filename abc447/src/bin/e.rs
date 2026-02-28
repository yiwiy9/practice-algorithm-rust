use ac_library::Dsu;
use proconio::{input, marker::Usize1};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut used = vec![false; m];
    for (i, &(u, v)) in uv.iter().enumerate().rev() {
        // クラスカル法
        if dsu.same(u, v) {
            continue;
        }
        used[i] = true;
        dsu.merge(u, v);
    }

    for i in 0..m {
        if used[i] {
            used[i] = false;
            break;
        }
    }

    let mut new_dsu = Dsu::new(n);
    for i in 0..m {
        if used[i] {
            new_dsu.merge(uv[i].0, uv[i].1);
        }
    }

    for i in 0..m {
        if !used[i] && new_dsu.same(uv[i].0, uv[i].1) {
            used[i] = true;
        }
    }

    let mut ans = 0;
    for i in 0..m {
        if !used[i] {
            ans += mod_pow(2, i + 1, MOD);
            ans %= MOD;
        }
    }
    println!("{}", ans);
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let base = base % modulo;
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}
