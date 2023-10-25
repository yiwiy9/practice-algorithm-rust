use num::integer::gcd;
use proconio::input;

/**
 * https://atcoder.jp/contests/abc177/tasks/abc177_e
 * https://atcoder.jp/contests/abc177/editorial/82
 *
 * 高速素因数分解
 * 前計算としてエラトステネスの篩を行い、「その数をふるい落とした素数」を配列 D に記録します。
 * 例えば D[4]=D[6]=2,D[35]=5 です。x が素数のときは D[x]=x としておきます。この配列はエラトステネスの篩と同様 O(AloglogA) で構築できます。
 *
 * D[x] は x を割り切る最小の素数なので、この配列 D を利用すると素因数分解を行うときに「試し割り」をする必要がなくなり(D[x]で割ればよい)、1つの数の素因数分解が素因数の個数である O(logA) でできるようになります。
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let min_divisors = eratosthenes_sieve(1_000_000);

    let mut cnt = vec![0; 1_000_000 + 1];
    a.iter().for_each(|&a_i| {
        let mut a_mut = a_i;
        while a_mut > 1 {
            let div = min_divisors[a_mut];
            cnt[div] += 1;
            while a_mut % div == 0 {
                a_mut /= div;
            }
        }
    });

    if cnt.iter().skip(2).all(|&num| num < 2) {
        println!("pairwise coprime");
        return;
    }

    if a.iter().fold(a[0], |acc, a_i| gcd(acc, *a_i)) == 1 {
        println!("setwise coprime");
        return;
    }

    println!("not coprime");
}

pub fn eratosthenes_sieve(n: usize) -> Vec<usize> {
    let mut min_divisors = vec![1; n + 1];
    for i in 2..=n {
        if min_divisors[i] == 1 {
            min_divisors[i] = i;
            let mut j = i * 2;
            while j <= n {
                if min_divisors[j] == 1 {
                    min_divisors[j] = i;
                }
                j += i;
            }
        }
    }
    min_divisors
}
