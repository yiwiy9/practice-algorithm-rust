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
    for p11 in 1..=28 {
        if p11 >= h1 {
            break;
        }
        if p11 >= w1 {
            break;
        }

        for p12 in 1..=28 {
            if p11 + p12 >= h1 {
                break;
            }
            if p12 >= w2 {
                break;
            }

            for p21 in 1..=28 {
                if p21 >= h2 {
                    break;
                }
                if p11 + p21 >= w1 {
                    break;
                }

                for p22 in 1..=28 {
                    if p21 + p22 >= h2 {
                        break;
                    }
                    if p12 + p22 >= w2 {
                        break;
                    }

                    let p13 = h1 - p11 - p12;
                    let p23 = h2 - p21 - p22;
                    let p31 = w1 - p11 - p21;
                    let p32 = w2 - p12 - p22;

                    if p31 + p32 < h3 {
                        let p33 = h3 - p31 - p32;
                        if p13 + p23 + p33 == w3 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
