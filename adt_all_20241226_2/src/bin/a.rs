use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut one_cnt = 0;
    let mut two_cnt = 0;
    let mut three_cnt = 0;
    for c in n {
        match c {
            '1' => one_cnt += 1,
            '2' => two_cnt += 1,
            '3' => three_cnt += 1,
            _ => (),
        }
    }

    println!(
        "{}",
        if one_cnt == 1 && two_cnt == 2 && three_cnt == 3 {
            "Yes"
        } else {
            "No"
        }
    );
}
