use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans: usize = 0;
    for mut num in 1..=n {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        if sum == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
