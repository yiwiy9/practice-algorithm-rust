use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }

    let mut mid = d.iter().sum::<usize>();
    mid += 1;
    mid /= 2;

    let mut cnt = 0;
    for i in 1..=m {
        for j in 1..=d[i - 1] {
            cnt += 1;
            if cnt == mid {
                println!("{} {}", i, j);
                return;
            }
        }
    }
}
