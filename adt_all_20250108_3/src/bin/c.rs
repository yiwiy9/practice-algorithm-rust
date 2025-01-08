use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for i in 0..=n {
        let mut ok = false;
        for j in 1..=9 {
            if n % j == 0 && i % (n / j) == 0 {
                ans.push(std::char::from_digit(j as u32, 10).unwrap());
                ok = true;
                break;
            }
        }
        if !ok {
            ans.push('-');
        }
    }

    println!("{}", ans.iter().join(""));
}
