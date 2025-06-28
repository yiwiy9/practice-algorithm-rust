use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut ok_a = true;
    let mut ok_b = true;
    for i in 1..n {
        let mut new_ok_a = false;
        let mut new_ok_b = false;
        if ok_a {
            if (a[i - 1] - a[i]).abs() <= k {
                new_ok_a = true;
            }
            if (a[i - 1] - b[i]).abs() <= k {
                new_ok_b = true;
            }
        }
        if ok_b {
            if (b[i - 1] - a[i]).abs() <= k {
                new_ok_a = true;
            }
            if (b[i - 1] - b[i]).abs() <= k {
                new_ok_b = true;
            }
        }
        if !new_ok_a && !new_ok_b {
            println!("No");
            return;
        }
        ok_a = new_ok_a;
        ok_b = new_ok_b;
    }

    println!("Yes");
}
