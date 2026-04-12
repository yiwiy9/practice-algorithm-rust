use proconio::input;

fn main() {
    input! {
        t: usize,
        x: i32,
        a: [i32; t+1],
    }

    let mut stack = vec![(0, a[0])];
    for i in 1..=t {
        if (a[i] - stack.last().unwrap().1).abs() >= x {
            stack.push((i, a[i]));
        }
    }

    for (i, j) in &stack {
        println!("{} {}", i, j);
    }
}
