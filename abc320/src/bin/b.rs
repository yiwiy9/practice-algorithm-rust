use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 1;
    let n = s.len();
    for start in 0..n {
        for end in start + 1..n {
            let mut ok = true;
            let mut step = 0;
            while start + step <= end - step {
                if s[start + step] != s[end - step] {
                    ok = false;
                    break;
                }
                step += 1;
            }
            if ok {
                ans = ans.max(end - start + 1);
            }
        }
    }

    println!("{}", ans);
}
