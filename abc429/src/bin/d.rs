use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: usize,
        mut a: [usize; n],
    }
    a.sort();

    a = a.iter().map(|&a_i| a_i - a[0]).collect::<Vec<_>>();

    let mut b = vec![];
    let mut cur = a[0];
    let mut cnt = 1;
    for &a_i in a.iter().skip(1) {
        if cur != a_i {
            b.push((if cur == 0 { m } else { cur }, cnt));
            cur = a_i;
            cnt = 1;
        } else {
            cnt += 1;
        }
    }
    b.push((if cur == 0 { m } else { cur }, cnt));
    b.sort();

    let n_b = b.len();

    let mut left = 0;
    let mut current = 0;
    let mut ans = 0;
    for right in 0..2 * n_b {
        current += b[right % n_b].1;
        if current >= c {
            while left < n_b && current >= c {
                ans += (b[left].0 - if left == 0 { 0 } else { b[left - 1].0 }) * current;
                current -= b[left].1;
                left += 1;
            }

            if left == n_b {
                break;
            }
        }
    }

    println!("{}", ans);
}
