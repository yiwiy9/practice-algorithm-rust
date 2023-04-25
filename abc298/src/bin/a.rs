use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut good_cnt = 0;
    let mut bad_cnt = 0;
    for c in s {
        match c {
            'o' => good_cnt += 1,
            'x' => bad_cnt += 1,
            _ => continue,
        }
    }

    println!(
        "{}",
        if good_cnt > 0 && bad_cnt == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
