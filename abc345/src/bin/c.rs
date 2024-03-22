use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt_vec = vec![0; 26];
    for &c in &s {
        cnt_vec[(c as u8 - b'a') as usize] += 1;
    }

    let mut diff_vec = vec![0; 26];
    for i in 0..26 {
        if cnt_vec[i] == 0 {
            continue;
        }
        for j in 0..26 {
            if i != j {
                diff_vec[i] += cnt_vec[j];
            }
        }
    }

    let mut ans = 0_usize;
    for &c in &s {
        ans += diff_vec[(c as u8 - b'a') as usize];
    }
    ans /= 2;

    if cnt_vec.iter().any(|&x| x > 1) {
        ans += 1;
    }

    println!("{}", ans);
}
