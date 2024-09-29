use ac_library::Dsu;
use proconio::{input, marker::Chars};
use std::collections::HashSet;

const MOD: usize = 998244353;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut dsu = Dsu::new(h * w);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }
            if i > 0 && s[i - 1][j] == '#' {
                dsu.merge(i * w + j, (i - 1) * w + j);
            }
            if j > 0 && s[i][j - 1] == '#' {
                dsu.merge(i * w + j, i * w + j - 1);
            }
        }
    }

    let mut leader_set = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }
            leader_set.insert(dsu.leader(i * w + j));
        }
    }

    let mut red_cnt = 0;
    let mut new_connected_sum = 0;
    let connected_cnt = leader_set.len();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let mut new_leader_set = HashSet::new();
            if i > 0 && s[i - 1][j] == '#' {
                new_leader_set.insert(dsu.leader((i - 1) * w + j));
            }
            if i + 1 < h && s[i + 1][j] == '#' {
                new_leader_set.insert(dsu.leader((i + 1) * w + j));
            }
            if j > 0 && s[i][j - 1] == '#' {
                new_leader_set.insert(dsu.leader(i * w + j - 1));
            }
            if j + 1 < w && s[i][j + 1] == '#' {
                new_leader_set.insert(dsu.leader(i * w + j + 1));
            }

            red_cnt += 1;
            new_connected_sum += connected_cnt - new_leader_set.len() + 1;
            new_connected_sum %= MOD;
        }
    }

    println!("{}", new_connected_sum * mod_inv(red_cnt, MOD) % MOD);
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
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}
