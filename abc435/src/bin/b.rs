use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for l in 0..n {
        for r in l..n {
            let mut ok = true;
            let sum: usize = a[l..=r].iter().sum();
            for i in l..=r {
                if sum % a[i] == 0 {
                    ok = false;
                }
            }
            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
