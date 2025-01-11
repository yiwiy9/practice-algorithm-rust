use proconio::input;

fn main() {
    input! {
        t: usize,
        nm: [(i64,i64); t],
    }

    for &(n, m) in &nm {
        let charge_lines = (n + 3) / 4;
        let sub_m = m - (charge_lines * 8 - n);
        if sub_m < 0 {
            println!("{}", charge_lines);
        } else {
            println!("{}", charge_lines + (sub_m + 7) / 8);
        }
    }
}
