use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut compressed = vec![];
    let mut increasing = p[0] < p[1];
    let mut cnt = 0;
    for i in 1..n {
        if (p[i - 1] < p[i]) == increasing {
            cnt += 1;
        } else if compressed.is_empty() && !increasing {
            cnt = 1;
            increasing = !increasing;
        } else {
            compressed.push(cnt);
            cnt = 1;
            increasing = !increasing;
        }
    }
    compressed.push(cnt);

    let mut ans = 0_usize;
    for (i, &c) in compressed.iter().enumerate() {
        if i % 2 != 0 || i + 2 >= compressed.len() {
            continue;
        }
        ans += c * compressed[i + 2];
    }

    println!("{}", ans);
}
