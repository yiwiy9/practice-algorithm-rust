use proconio::input;

const UPPER: usize = 10000;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut ans = UPPER;
    for i in 0..UPPER {
        let x = a * i;
        if x > n {
            break;
        }
        for j in 0..UPPER - i {
            let y = b * j;
            if x + y > n {
                break;
            }
            if (n - x - y) % c == 0 {
                ans = ans.min(i + j + (n - x - y) / c);
            }
        }
    }

    println!("{}", ans);
}
