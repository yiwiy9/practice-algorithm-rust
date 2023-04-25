use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut count = 0;
    for i in 1..=n {
        let mut digit_sum = 0;
        let mut j = i;

        while j != 0 {
            digit_sum += j % 10;
            j /= 10;
        }

        if digit_sum >= a && digit_sum <= b {
            count += i;
        }
    }

    println!("{}", count);
}
