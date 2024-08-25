use proconio::input;

/**
 * https://note.com/kota_takahashi_/n/n54e837b3ec9d - 簡易
 * https://algo-logic.info/combinatorial-games/#google_vignette - 詳細
 * - Nim
 * - Grundy数
 *
 * https://atcoder.jp/contests/abc368/tasks/abc368_f
 * https://atcoder.jp/contests/abc368/editorial/10761
 *
 */
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut nim = vec![0; n];
    for i in 0..n {
        nim[i] = factor_counts(a[i]);
    }

    let mut xor_sum = 0;
    for &nim_i in &nim {
        xor_sum ^= nim_i;
    }

    println!("{}", if xor_sum == 0 { "Bruno" } else { "Anna" });
}

fn factor_counts(mut n: usize) -> usize {
    let mut factor_counts = 0;
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i != 0 {
            continue;
        }
        let mut count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        factor_counts += count
    }
    if n != 1 {
        factor_counts += 1;
    }
    factor_counts
}
