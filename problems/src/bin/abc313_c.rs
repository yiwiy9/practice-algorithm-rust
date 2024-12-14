use proconio::input;

fn main() {
    input! {
        n: i64,
        mut a: [i64; n],
    }

    let sum_a = a.iter().sum::<i64>();
    let num_small = sum_a / n;
    let num_large = num_small + 1;
    let num_large_count = sum_a % n;
    let num_small_count = n - num_large_count;

    a.sort();

    let mut ans = 0;
    for i in 0..num_small_count {
        ans += (a[i as usize] - num_small).abs();
    }
    for i in num_small_count..n {
        ans += (a[i as usize] - num_large).abs();
    }

    println!("{}", ans / 2);
}
