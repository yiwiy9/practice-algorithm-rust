use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut arr = vec![0; 5];
    arr[a] += 1;
    arr[b] += 1;
    arr[c] += 1;
    arr[d] += 1;

    println!("{}", arr.iter().map(|&x| x / 2).sum::<usize>());
}
