use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_max = *a.iter().max().unwrap();
    let mut cnt = vec![0; a_max + 1];
    for &a_i in &a {
        cnt[a_i] += 1;
    }

    // 調和級数なので O(N log N) で解ける
    // => 1 + 1/2 + 1/3 + ... + 1/N = O(log N)
    let mut ans = 0_usize;
    for i in 1..=a_max {
        for j in 1..=a_max {
            if i * j > a_max {
                break;
            }
            ans += cnt[i] * cnt[j] * cnt[i * j];
        }
    }

    println!("{}", ans);
}
