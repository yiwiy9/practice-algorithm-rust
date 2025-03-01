use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a_i = vec![];
    for i in 0..n {
        let mut b_i = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                b_i.push(1);
            } else {
                b_i.push(a_i[j - 1] + a_i[j]);
            }
        }
        println!("{}", b_i.iter().join(" "));
        a_i = b_i;
    }
}
