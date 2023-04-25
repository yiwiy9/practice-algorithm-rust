use proconio::input;
use std::collections::VecDeque;

const MOD: usize = 998244353;

fn main() {
    input! {
        q: usize,
    }
    let mut ans = 1;
    let mut s = VecDeque::new();
    s.push_back(1);

    for _q in 0..q {
        input! {
            num: usize,
        }
        match num {
            1 => {
                input! {
                    x: usize,
                }
                ans *= 10;
                ans += x;
                ans %= MOD;
                s.push_back(x);
            }
            2 => {
                ans -= (mod_pow(10, s.len() - 1, MOD) * s.front().unwrap()) % MOD;
                ans += MOD;
                ans %= MOD;
                s.pop_front();
            }
            3 => println!("{}", ans),
            _ => unreachable!(),
        }
    }
}

pub fn mod_pow(base: usize, exp: usize, modulo: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let mut result = mod_pow(base * base % modulo, exp / 2, modulo);
    if exp % 2 == 1 {
        result *= base;
        result %= modulo;
    }
    result
}
