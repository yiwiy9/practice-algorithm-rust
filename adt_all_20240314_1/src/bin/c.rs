use proconio::input;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }

    let mut i = 0;
    let mut cur_g = 0;
    let mut cur_m = 0;
    while i < k {
        if cur_g == g {
            cur_g = 0;
        } else if cur_m == 0 {
            cur_m = m;
        } else if cur_g + cur_m >= g {
            cur_m -= g - cur_g;
            cur_g = g;
        } else {
            cur_g += cur_m;
            cur_m = 0;
        }
        i += 1;
    }

    println!("{} {}", cur_g, cur_m);
}
