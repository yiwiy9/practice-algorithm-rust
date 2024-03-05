use proconio::input;

fn main() {
    input! {
     h1: usize,
     h2: usize,
     h3: usize,
     w1: usize,
     w2: usize,
     w3: usize,
    }

    let mut ans = 0;
    for x1 in 1..=28 {
        for x2 in 1..=28 {
            if x1 + x2 >= h1 {
                continue;
            }
            let x3 = h1 - x1 - x2;
            for y1 in 1..=28 {
                if x1 + y1 >= w1 {
                    continue;
                }
                let z1 = w1 - x1 - y1;
                for y2 in 1..=28 {
                    if y1 + y2 >= h2 {
                        continue;
                    }
                    let y3 = h2 - y1 - y2;

                    if x2 + y2 >= w2 {
                        continue;
                    }
                    let z2 = w2 - x2 - y2;

                    if z1 + z2 >= h3 {
                        continue;
                    }
                    let z3 = h3 - z1 - z2;

                    if x3 + y3 + z3 == w3 {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
