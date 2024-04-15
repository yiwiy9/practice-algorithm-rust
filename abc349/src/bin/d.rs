use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let mut x_vec = vec![];
    let mut x = 1;
    x_vec.push(x);
    for _ in 0..60 {
        x *= 2;
        if x > r {
            break;
        }
        x_vec.push(x);
    }
    x_vec.reverse();

    let mut ans = vec![];
    let mut cur = l;
    while cur < r {
        for &x in &x_vec {
            if cur % x != 0 {
                continue;
            }

            let j = cur / x;
            if x * (j + 1) > r {
                continue;
            }

            ans.push((cur, x * (j + 1)));
            cur = x * (j + 1);
            break;
        }
    }

    println!("{}", ans.len());
    for (a, b) in ans {
        println!("{} {}", a, b);
    }
}
