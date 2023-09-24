use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n-1],
    }

    let mut ans = -1;
    for i in 0..=100 {
        let mut b = a.clone();
        b.push(i);
        b.sort();
        let sum: usize = b.iter().skip(1).take(n - 2).sum();
        if sum >= x {
            ans = i as i32;
            break;
        }
    }

    println!("{}", ans);
}
