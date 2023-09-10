use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    for i in 0..=n {
        let mut cur = '-';
        for j in 1..=9 {
            if n % j != 0 {
                continue;
            }
            if i % (n / j) == 0 {
                cur = std::char::from_digit(j as u32, 10).unwrap();
                break;
            }
        }
        ans.push(cur);
    }

    println!("{}", ans.iter().join(""));
}
