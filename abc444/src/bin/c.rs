use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut ans: Vec<usize> = vec![];

    if n % 2 == 0 {
        let mut ok = true;
        let num = a[0] + a[n - 1];
        for i in 0..n / 2 {
            if a[i] + a[n - i - 1] != num {
                ok = false;
            }
        }
        if ok {
            ans.push(num);
        }
    }

    let num = a[n - 1];
    let a = a.iter().filter(|&&c| c != num).collect_vec();
    let n = a.len();
    if n > 0 && n % 2 == 0 {
        let mut ok = true;
        let num = a[0] + a[n - 1];
        for i in 0..n / 2 {
            if a[i] + a[n - i - 1] != num {
                ok = false;
            }
        }
        if ok {
            ans.push(num);
        }
    }

    if n == 0 {
        ans.push(num);
    }

    ans.sort();

    println!("{}", ans.iter().join(" "));
}
