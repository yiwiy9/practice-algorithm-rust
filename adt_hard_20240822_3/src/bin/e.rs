use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
         n: usize,
    }

    println!("{}", rec(n, &mut HashMap::new()));
}

fn rec(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n < 2 {
        return 0;
    }
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let upper = if n % 2 == 0 { n / 2 } else { (n + 1) / 2 };
    let lower = n / 2;

    let mut ans = upper + lower;
    ans += rec(upper, memo);
    ans += rec(lower, memo);

    memo.insert(n, ans);
    ans
}
