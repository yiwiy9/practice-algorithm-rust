use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0; a.iter().max().unwrap() + 1];
    for &x in &a {
        cnt[x] += 1;
    }

    // 調和級数
    let mut ans = 0_usize;
    for i in 1..=cnt.len() {
        for j in 1..=cnt.len() {
            if i * j >= cnt.len() {
                break;
            }
            ans += cnt[i] * cnt[j] * cnt[i * j];
        }
    }

    println!("{}", ans);
}
