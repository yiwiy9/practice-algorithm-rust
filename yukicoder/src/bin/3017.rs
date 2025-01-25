use proconio::input;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut stack = vec![(INF, 1)];
    let mut water = 0;
    let mut green = 0;
    for (i, &h_i) in h.iter().enumerate() {
        while let Some(&(top_h, top_c)) = stack.last() {
            if top_h > h_i {
                break;
            }
            stack.pop();
            if top_c == 0 {
                water -= top_h;
            } else {
                green -= top_h;
            }
        }

        if let Some(&(_, top_c)) = stack.last() {
            if top_c != i % 2 {
                stack.push((h_i, i % 2));
                if i % 2 == 0 {
                    water += h_i;
                } else {
                    green += h_i;
                }
            }
        }

        println!("{}", (water - green).max(0));
    }
}
