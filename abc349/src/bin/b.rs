use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt_vec = vec![0; 26];
    for c in s {
        cnt_vec[(c as u8 - b'a') as usize] += 1;
    }

    let mut kind_cnt_vec = vec![0; 101];
    for cnt in cnt_vec {
        kind_cnt_vec[cnt] += 1;
    }

    println!(
        "{}",
        if kind_cnt_vec.iter().skip(1).all(|&x| x == 0 || x == 2) {
            "Yes"
        } else {
            "No"
        }
    );
}
