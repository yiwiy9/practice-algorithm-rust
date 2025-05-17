use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; n]; m],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut set = std::collections::HashSet::new();
        for a_i in &a {
            for (idx, &a_ij) in a_i.iter().enumerate() {
                if a_ij == i {
                    if idx > 0 {
                        set.insert(a_i[idx - 1]);
                    }
                    if idx < n - 1 {
                        set.insert(a_i[idx + 1]);
                    }
                    break;
                }
            }
        }
        let mut cnt = 0;
        for j in i + 1..n {
            if !set.contains(&j) {
                cnt += 1;
            }
        }
        ans += cnt;
    }

    println!("{}", ans);
}
