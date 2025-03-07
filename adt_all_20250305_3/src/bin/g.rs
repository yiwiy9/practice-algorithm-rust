use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..2 * n - 1 {
        for j in i + 1..2 * n {
            input! {
                a_ij: usize,
            }
            a[i][j] = a_ij;
        }
    }

    let mut max_xor_sum = 0;
    let mut used = HashSet::new();
    rec(n, &a, &mut used, &mut max_xor_sum, 0);

    println!("{}", max_xor_sum);
}

fn rec(
    n: usize,
    a: &Vec<Vec<usize>>,
    used: &mut HashSet<usize>,
    max_xor_sum: &mut usize,
    xor_sum: usize,
) {
    if used.len() == 2 * n {
        *max_xor_sum = std::cmp::max(*max_xor_sum, xor_sum);
        return;
    }

    let mut next_i = 2 * n;
    for i in 0..2 * n - 1 {
        if !used.contains(&i) {
            next_i = i;
            used.insert(next_i);
            break;
        }
    }

    for j in next_i + 1..2 * n {
        if !used.contains(&j) {
            used.insert(j);
            rec(n, a, used, max_xor_sum, xor_sum ^ a[next_i][j]);
            used.remove(&j);
        }
    }

    used.remove(&next_i);
}
