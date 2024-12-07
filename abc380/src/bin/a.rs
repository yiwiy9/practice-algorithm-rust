use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut num_cnt = vec![0; 10];
    for &c in &n {
        num_cnt[c as usize - '0' as usize] += 1;
    }

    println!(
        "{}",
        if num_cnt[1] == 1 && num_cnt[2] == 2 && num_cnt[3] == 3 {
            "Yes"
        } else {
            "No"
        }
    );
}
