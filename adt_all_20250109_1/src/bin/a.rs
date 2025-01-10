use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut for_cnt = 0;
    let mut against_cnt = 0;
    for i in 0..n {
        if s[i] == "For" {
            for_cnt += 1;
        } else {
            against_cnt += 1;
        }
    }

    println!("{}", if for_cnt > against_cnt { "Yes" } else { "No" });
}
