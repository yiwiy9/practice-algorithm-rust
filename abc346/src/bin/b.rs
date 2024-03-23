use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    let s = String::from("wbwbwwbwbwbw");
    let ss: Vec<char> = s.chars().collect();

    let mut ok = false;
    for i in 0..10_020 {
        let mut w_cnt = 0;
        let mut b_cnt = 0;
        for j in 0..w + b {
            if ss[(i + j) % 12] == 'w' {
                w_cnt += 1;
            } else {
                b_cnt += 1;
            }
        }
        if w_cnt == w && b_cnt == b {
            ok = true;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
