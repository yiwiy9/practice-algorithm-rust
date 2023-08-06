use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        k: usize,
    }

    let mut a = vec![0; n];

    let mut first_sum = 0;
    for i in 0..k + 1 {
        println!("? {}", (1..=k + 1).filter(|&num| num != i + 1).join(" "));
        stdout().flush().unwrap();

        input! {
            from &mut source,
            t: usize,
        }
        a[i] = t;
        first_sum ^= a[i];
    }

    for t_i in a.iter_mut().take(k + 1) {
        *t_i ^= first_sum
    }

    let mut k_vec = (1..=k).collect_vec();
    let k_sum = a.iter().take(k - 1).fold(0, |bef, cur| bef ^ cur);
    for i in k + 1..n {
        k_vec[k - 1] = i + 1;
        println!("? {}", k_vec.iter().join(" "));
        stdout().flush().unwrap();

        input! {
            from &mut source,
            t: usize,
        }
        a[i] = t ^ k_sum;
    }

    println!("! {}", a.iter().join(" "));
}
