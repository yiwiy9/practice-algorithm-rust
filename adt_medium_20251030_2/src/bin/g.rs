use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 0..2 * n - 1 {
        input! {
            a_i: [usize; 2*n-i-1],
        }
        a.push(a_i);
    }

    let mut ans = 0;
    let mut set = HashSet::new();
    rec(n, &a, &mut set, &mut ans, 0);

    println!("{}", ans);
}

fn rec(n: usize, a: &[Vec<usize>], used: &mut HashSet<usize>, max_xor: &mut usize, cur_xor: usize) {
    if used.len() == 2 * n {
        if cur_xor > *max_xor {
            *max_xor = cur_xor;
        }
        return;
    }

    let mut i = 0;
    for ii in 0..2 * n {
        if !used.contains(&ii) {
            i = ii;
            used.insert(ii);
            break;
        }
    }

    for j in 0..2 * n {
        if !used.contains(&j) {
            used.insert(j);
            rec(
                n,
                a,
                used,
                max_xor,
                cur_xor ^ a[i.min(j)][i.max(j) - i.min(j) - 1],
            );
            used.remove(&j);
        }
    }

    used.remove(&i);
}
