use proconio::input;
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        mut a: [i64; n],
    }

    let mut l_a = vec![0; n + 1];
    for i in 0..n {
        let all_l = l * (i as i64 + 1);
        l_a[i + 1] = all_l.min(l_a[i] + a[i]);
    }

    a.reverse();
    let mut r_a = vec![0; n + 1];
    for i in 0..n {
        let all_r = r * (i as i64 + 1);
        r_a[i + 1] = all_r.min(r_a[i] + a[i]);
    }

    let mut ans = INF;
    for i in 0..=n {
        let l_idx = i;
        let r_idx = n - i;
        ans = ans.min(l_a[l_idx] + r_a[r_idx]);
    }

    println!("{}", ans);
}
