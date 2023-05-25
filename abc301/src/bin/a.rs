use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut t_cnt = 0;
    let mut a_cnt = 0;

    let half = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };

    for c in &s {
        match c {
            'T' => t_cnt += 1,
            'A' => a_cnt += 1,
            _ => unreachable!(),
        }
        if t_cnt == half || a_cnt == half {
            break;
        }
    }

    println!("{}", if t_cnt > a_cnt { 'T' } else { 'A' });
}
