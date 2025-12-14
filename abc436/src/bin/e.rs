use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut ans = 0_usize;
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }

        seen[i] = true;
        let mut cnt = 1;
        let mut j = p[i];
        while i != j {
            seen[j] = true;
            cnt += 1;
            j = p[j];
        }
        if cnt > 1 {
            ans += cnt * (cnt - 1) / 2;
        }
    }

    println!("{}", ans);
}
