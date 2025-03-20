use ac_library::Dsu;
use proconio::{input, marker::Chars};
use std::collections::HashSet;

const MOD: usize = 998244353;
const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut dsu = Dsu::new(h * w);
    let mut red_points = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                if i > 0 && s[i - 1][j] == '#' {
                    dsu.merge(i * w + j, (i - 1) * w + j);
                }
                if j > 0 && s[i][j - 1] == '#' {
                    dsu.merge(i * w + j, i * w + j - 1);
                }
            } else {
                red_points.push((i, j));
            }
        }
    }

    let mut leaders = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            let leader = dsu.leader(i * w + j);
            leaders.insert(leader);
        }
    }
    let default_cnt = leaders.len();

    let mut ans = 0;
    for &(i, j) in &red_points {
        let mut surrounding_leaders = HashSet::new();
        for k in 0..4 {
            let ni = i as i32 + DX[k];
            let nj = j as i32 + DY[k];
            if ni < 0 || nj < 0 || ni >= h as i32 || nj >= w as i32 {
                continue;
            }
            if s[ni as usize][nj as usize] == '.' {
                continue;
            }
            let leader = dsu.leader(ni as usize * w + nj as usize);
            surrounding_leaders.insert(leader);
        }

        ans += default_cnt - surrounding_leaders.len() + 1;
        ans %= MOD;
    }

    ans *= mod_inv(red_points.len(), MOD);
    ans %= MOD;

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
pub fn mod_inv(num: usize, modulo: usize) -> usize {
    mod_pow(num, modulo - 2, modulo)
}
