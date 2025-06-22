use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut queries = vec![];
    for _ in 0..q {
        input! {
            op: u8,
        }

        match op {
            1 => {
                input! {
                    p: Usize1,
                }
                queries.push((1, p, "".to_string()));
            }
            2 => {
                input! {
                    p: Usize1,
                    s: String,
                }
                queries.push((2, p, s));
            }
            3 => {
                input! {
                    p: Usize1,
                }
                queries.push((3, p, "".to_string()));
            }
            _ => unreachable!(),
        }
    }

    let mut cur = n;
    let mut ans = vec![];
    for (op, p, s) in queries.iter().rev() {
        if cur == n {
            match op {
                3 => {
                    cur = *p;
                }
                _ => continue,
            }
        } else {
            match op {
                1 => {
                    if *p == cur {
                        cur = n;
                    }
                }
                2 => {
                    if *p == cur {
                        ans.push(s.clone());
                    }
                }
                _ => continue,
            }
        }
    }

    ans.reverse();
    println!("{}", ans.join(""));
}
