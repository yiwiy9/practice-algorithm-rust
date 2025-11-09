use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [usize; n],
        mut b: [usize; m],
    }
    h.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut j = 0;
    for &b_i in &b {
        while j < n && h[j] > b_i {
            j += 1;
        }

        if j == n {
            break;
        }

        ans += 1;
        j += 1;
    }

    println!("{}", if ans >= k { "Yes" } else { "No" });
}
