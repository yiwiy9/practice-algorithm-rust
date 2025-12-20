use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        xv: [(usize,usize); n],
    }

    let mut left = 0;
    let mut right: usize = 1 << 60;
    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut max_display = vec![];
        for &(x, v) in &xv {
            if v >= mid {
                max_display.push(x);
            }
        }

        if max_display.len() < m {
            right = mid;
            continue;
        }

        max_display.sort();

        let mut display = vec![max_display[0]];
        for i in 1..max_display.len() {
            if max_display[i] - display.last().unwrap() >= d {
                display.push(max_display[i]);
            }
        }

        if display.len() >= m {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", if left == 0 { -1 } else { left as i64 });
}
