use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let is_prime = eratosthenes_sieve(200);
    let mut takahashi_win = false;
    for i in a..=b {
        let mut can_win = false;
        for j in c..=d {
            if is_prime[i + j] {
                can_win = true;
                break;
            }
        }
        if !can_win {
            takahashi_win = true;
            break;
        }
    }

    println!("{}", if takahashi_win { "Takahashi" } else { "Aoki" });
}

pub fn eratosthenes_sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if is_prime[i] {
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    is_prime
}
