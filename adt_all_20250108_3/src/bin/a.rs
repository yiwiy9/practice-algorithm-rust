use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut t_cnt = 0;
    let mut a_cnt = 0;
    for &c in &s {
        if c == 'T' {
            t_cnt += 1;
        } else {
            a_cnt += 1;
        }
    }

    println!(
        "{}",
        if t_cnt == a_cnt {
            if s.last() == Some(&'T') {
                "A"
            } else {
                "T"
            }
        } else if t_cnt > a_cnt {
            "T"
        } else {
            "A"
        }
    );
}
