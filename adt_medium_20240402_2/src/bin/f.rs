use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [Chars; n],
    }

    let mut ans = false;

    // next_permutation()
    permutohedron::heap_recursive(&mut a, |a| {
        let mut ok = true;
        for i in 0..n - 1 {
            let mut diff_cnt = 0;
            for j in 0..m {
                if a[i][j] != a[i + 1][j] {
                    diff_cnt += 1;
                }
            }
            if diff_cnt > 1 {
                ok = false;
                break;
            }
        }

        if ok {
            ans = true;
        }
    });

    println!("{}", if ans { "Yes" } else { "No" });
}
