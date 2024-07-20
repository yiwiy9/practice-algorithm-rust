use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut exclude_square = vec![0; 2000001];
    for &x in &a {
        let mut max_square_left = 0;
        for i in 1..=((x as f64).sqrt() as usize) {
            if x % (i * i) == 0 {
                max_square_left = x / (i * i);
            }
        }
        exclude_square[max_square_left] += 1;
    }

    let mut ans = exclude_square[0] * (n - exclude_square[0]);
    for i in 0..=2000000 {
        if exclude_square[i] > 1 {
            ans += exclude_square[i] * (exclude_square[i] - 1) / 2;
        }
    }

    println!("{}", ans);
}
