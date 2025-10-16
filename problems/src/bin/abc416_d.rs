use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            mut a: [usize; n],
            mut b: [usize; n],
        }

        a.sort_by(|a, b| b.cmp(a));
        b.sort();

        let mut c = 0;
        let mut idx = 0;
        for &a_k in &a {
            while idx < n && b[idx] + a_k < m {
                idx += 1;
            }
            if idx >= n {
                break;
            }
            c += 1;
            idx += 1;
        }

        println!(
            "{}",
            a.iter().sum::<usize>() + b.iter().sum::<usize>() - m * c
        );
    }
}
