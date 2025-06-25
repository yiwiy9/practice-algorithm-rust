use proconio::{input_interactive, marker::Usize1};

fn main() {
    input_interactive! {
        n: usize,
    }

    let mut used = vec![false; 2 * n + 1];
    for i in 0..2 * n + 1 {
        if used[i] {
            continue;
        }
        println!("{}", i + 1);
        used[i] = true;

        input_interactive! {
            a: Usize1,
        }
        used[a] = true;
    }
}
