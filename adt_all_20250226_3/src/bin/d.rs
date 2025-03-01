use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    let s = "wbwbwwbwbwbw".to_string();
    let inf_s = s.repeat(200).chars().collect::<Vec<char>>();

    for i in 0..s.len() {
        let mut w_cnt = 0;
        let mut b_cnt = 0;
        for j in i..w + b + i {
            if inf_s[j] == 'w' {
                w_cnt += 1;
            } else {
                b_cnt += 1;
            }
        }
        if w_cnt == w && b_cnt == b {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
