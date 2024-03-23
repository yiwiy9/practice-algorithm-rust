use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        r: [usize; n],
    }

    let target_iter = if c.iter().any(|&x| x == t) {
        c.iter()
            .enumerate()
            .filter(|&(_, &x)| x == t)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    } else {
        c.iter()
            .enumerate()
            .filter(|&(_, &x)| x == c[0])
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
    };

    let mut max_i = 0;
    let mut max_r = 0;
    for i in target_iter {
        if r[i] > max_r {
            max_i = i;
            max_r = r[i];
        }
    }

    println!("{}", max_i + 1);
}
