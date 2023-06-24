use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut is_r = true;
    let mut r_cnt = 0;
    let mut l_cnt = 0;
    let mut a = vec![];
    for c in &s {
        if is_r {
            if *c == 'R' {
                r_cnt += 1;
            } else {
                l_cnt = 1;
                is_r = false;
            }
        } else if *c == 'R' {
            a.push((r_cnt, l_cnt));
            r_cnt = 1;
            is_r = true;
        } else {
            l_cnt += 1;
        }
    }
    a.push((r_cnt, l_cnt));

    let mut ans = vec![];
    for &(r_cnt, l_cnt) in &a {
        for _ in 0..r_cnt - 1 {
            ans.push(0)
        }

        if (r_cnt + l_cnt) % 2 == 0 {
            ans.push((r_cnt + l_cnt) / 2);
            ans.push((r_cnt + l_cnt) / 2);
        } else if r_cnt > l_cnt {
            if r_cnt % 2 == 0 {
                ans.push((r_cnt + l_cnt) / 2);
                ans.push((r_cnt + l_cnt) / 2 + 1);
            } else {
                ans.push((r_cnt + l_cnt) / 2 + 1);
                ans.push((r_cnt + l_cnt) / 2);
            }
        } else if l_cnt % 2 == 0 {
            ans.push((r_cnt + l_cnt) / 2 + 1);
            ans.push((r_cnt + l_cnt) / 2);
        } else {
            ans.push((r_cnt + l_cnt) / 2);
            ans.push((r_cnt + l_cnt) / 2 + 1);
        }

        for _ in 0..l_cnt - 1 {
            ans.push(0)
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
