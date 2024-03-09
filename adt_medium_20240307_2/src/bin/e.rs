use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let sum = a.iter().sum::<usize>();
    let target_lower = sum / n;
    let target_upper = target_lower + 1;

    let mut lower_sum = 0;
    let mut upper_sum = 0;
    for a_i in a {
        if a_i <= target_lower {
            lower_sum += target_lower - a_i;
        } else {
            upper_sum += a_i - target_upper;
        }
    }

    println!("{}", lower_sum.max(upper_sum));
}
