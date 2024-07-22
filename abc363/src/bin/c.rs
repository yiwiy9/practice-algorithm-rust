use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars,
    }

    let mut set = HashSet::new();
    // next_permutation()
    permutohedron::heap_recursive(&mut s, |t| {
        let t_clone = t.to_vec();
        set.insert(t_clone);
    });

    let mut ans = 0;
    for t in set {
        let mut exist = false;
        for i in 0..=(n - k) {
            let mut ok = true;
            for j in 0..k {
                if t[i + j] != t[i + k - 1 - j] {
                    ok = false;
                    break;
                }
            }
            if ok {
                exist = true;
                break;
            }
        }
        if !exist {
            ans += 1;
        }
    }

    println!("{}", ans);
}
