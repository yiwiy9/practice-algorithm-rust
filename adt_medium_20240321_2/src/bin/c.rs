use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut a: [usize; n],
    }

    let mut good_nums = vec![];
    for i in 0..n {
        good_nums.push(a[i]);

        for j in i + 1..n {
            good_nums.push(a[i] + a[j]);

            for k in j + 1..n {
                good_nums.push(a[i] + a[j] + a[k]);
            }
        }
    }

    println!("{}", good_nums.iter().unique().filter(|&&x| x <= w).count());
}
