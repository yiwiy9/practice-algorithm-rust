use proconio::input;

fn main() {
    input! {
        n: usize,
        c: i64,
        abc: [(usize, usize, i64); n],
    }

    let mut not_prime_days = vec![];
    abc.iter().for_each(|&(a_i, b_i, c_i)| {
        not_prime_days.push((a_i, c_i)); // 利用開始日 +c_i円
        not_prime_days.push((b_i + 1, -c_i)); // 利用終了日 -c_i円
    });
    not_prime_days.sort();

    let mut not_prime_price = 0;
    let mut ans = 0;
    for (i, &(day, diff_price)) in not_prime_days.iter().enumerate() {
        if i + 1 < not_prime_days.len() {
            not_prime_price += diff_price;
            let price = not_prime_price.min(c);
            ans += (not_prime_days[i + 1].0 - day) as i64 * price;
        }
    }

    println!("{}", ans);
}
