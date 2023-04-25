use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }

    let mut diff_sum = 0;
    for (a_i, b_i) in a.iter().zip(b.iter()).take(n) {
        diff_sum += (a_i - b_i).abs();
    }

    println!(
        "{}",
        if diff_sum <= k && (diff_sum - k) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
