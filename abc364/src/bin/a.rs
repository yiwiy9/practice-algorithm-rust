use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut sweet_continuous_cnt = 0;
    for s_i in &s {
        if sweet_continuous_cnt >= 2 {
            println!("No");
            return;
        }
        if s_i == "sweet" {
            sweet_continuous_cnt += 1;
        } else {
            sweet_continuous_cnt = 0;
        }
    }

    println!("Yes");
}
