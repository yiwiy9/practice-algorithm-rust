use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    let mut ans = 0;

    for s_i in &s {
        for t_i in &t {
            let mut ok = true;
            for x in 0..3 {
                if s_i[x + 3] != t_i[x] {
                    ok = false
                }
            }
            if ok {
                ans += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
