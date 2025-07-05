use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: u32,
            a: [usize; 2_u32.pow(n)],
        }
        let a = rec_sort(&a);
        println!("{}", a.iter().join(" "));
    }
}

fn rec_sort(a: &Vec<usize>) -> Vec<usize> {
    if a.len() <= 1 {
        return a.clone();
    }

    let mid = a.len() / 2;

    let left = rec_sort(&a[0..mid].to_vec());
    let right = rec_sort(&a[mid..].to_vec());

    if left.iter().min().unwrap() < right.iter().min().unwrap() {
        let mut merged = left;
        merged.extend(right);
        return merged;
    } else {
        let mut merged = right;
        merged.extend(left);
        return merged;
    }
}
