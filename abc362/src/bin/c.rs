use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64,i64); n],
    }

    let mut x = vec![];
    for &(l, _) in &lr {
        x.push(l);
    }

    let mut ans = x.clone();
    let mut sum = -x.iter().sum::<i64>();
    if sum < 0 {
        println!("No");
        return;
    }

    for (i, x_i) in x.iter().enumerate() {
        let diff = lr[i].1 - x_i;
        if sum - diff > 0 {
            ans[i] += diff;
            sum -= diff;
        } else {
            ans[i] += sum;
            sum = 0;
            break;
        }
    }

    if sum == 0 {
        println!("Yes");
        println!("{}", ans.iter().join(" "));
    } else {
        println!("No");
    }
}
