use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut a: [i64; n],
        }

        if a.iter().all(|&x| x.abs() == a[0].abs()) {
            let plus_count = a.iter().filter(|&&x| x > 0).count() as i64;
            let minus_count = a.iter().filter(|&&x| x < 0).count() as i64;
            if plus_count == n as i64
                || minus_count == n as i64
                || (plus_count - minus_count).abs() <= 1
            {
                println!("Yes");
            } else {
                println!("No");
            }
            continue;
        }

        a.sort_by_key(|x| (x.abs()));

        let mother = a[0];
        let child = a[1];
        let mut ok = true;
        for i in 2..n {
            let child2 = a[i - 1] * child;
            if child2 % mother != 0 {
                ok = false;
                break;
            }
            if a[i] != child2 / mother {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
