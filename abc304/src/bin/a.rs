use proconio::input;

fn main() {
    input! {
        n: usize,
        s_a: [(String,usize); n],
    }

    let mut min_i = n;
    let mut min = 1 << 30;
    for (i, (_, a)) in s_a.iter().enumerate() {
        if *a < min {
            min_i = i;
            min = *a;
        }
    }

    let mut circle_1 = vec![];
    let mut circle_2 = vec![];
    for (i, (s, _)) in s_a.iter().enumerate() {
        if i < min_i {
            circle_2.push(s);
        } else {
            circle_1.push(s);
        }
    }
    for s in &circle_1 {
        println!("{}", s);
    }
    for s in &circle_2 {
        println!("{}", s);
    }
}
