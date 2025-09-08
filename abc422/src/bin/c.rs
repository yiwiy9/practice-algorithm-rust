use proconio::input;

fn main() {
    input! {
        t: usize,
        abc: [(usize, usize, usize); t],
    }

    for &(a, b, c) in &abc {
        let mut left = 0;
        let mut right: usize = 1 << 30;
        while right - left > 1 {
            let mid = (left + right) / 2;

            let min_ac = a.min(c);

            if min_ac < mid {
                right = mid;
                continue;
            }

            let sub = min_ac - mid;
            let remains = 2 * sub + (a - min_ac) + (c - min_ac) + b;
            if mid <= remains {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left);
    }
}
