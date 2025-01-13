use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = vec![0; n];
    let mut sub_vec = vec![0; n];
    let mut sub = 0;
    for i in 0..n {
        let give = n - i - 1;
        sub += sub_vec[i];
        let take = i - sub;

        if a[i] + take >= give {
            b[i] = a[i] + take - give;
        } else {
            let minus = give - (a[i] + take);
            sub_vec[n - minus] += 1;
            b[i] = 0;
        }
    }

    println!("{}", b.iter().join(" "));
}
