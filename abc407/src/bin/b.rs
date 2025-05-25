use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    let mut cnt = 0.0;
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= x || (i - j).abs() >= y {
                cnt += 1.0;
            }
        }
    }

    println!("{:.12}", cnt / 36.0);
}
