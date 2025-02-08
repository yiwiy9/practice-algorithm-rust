use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        mut x: usize,
    }

    let a_sum = a.iter().sum::<usize>();
    let mut ans = x / a_sum * n;
    x %= a_sum;

    for a_i in a {
        ans += 1;
        if x < a_i {
            break;
        }
        x -= a_i;
    }

    println!("{}", ans);
}
