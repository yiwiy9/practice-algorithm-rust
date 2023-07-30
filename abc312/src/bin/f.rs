use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tx: [(usize, usize); n],
    }

    let mut opened_cans = vec![];
    let mut closed_cans = vec![];
    let mut openers = vec![];

    for &(t, x) in &tx {
        match t {
            0 => opened_cans.push(x),
            1 => closed_cans.push(x),
            2 => openers.push(x),
            _ => unreachable!(),
        }
    }

    opened_cans.sort_by(|a, b| b.cmp(a));
    closed_cans.sort();
    openers.sort();

    let mut opened_cans_sum = vec![0; n + 1];
    for i in 0..n {
        if i < opened_cans.len() {
            opened_cans_sum[i + 1] = opened_cans_sum[i] + opened_cans[i];
        } else {
            opened_cans_sum[i + 1] = opened_cans_sum[i];
        }
    }

    let mut opener_cnt = 0;
    let mut closed_cans_sum = vec![0; n + 1];
    for i in 0..n {
        if closed_cans.is_empty() {
            closed_cans_sum[i + 1] = closed_cans_sum[i];
            continue;
        }

        if opener_cnt == 0 {
            if !openers.is_empty() {
                opener_cnt = openers.pop().unwrap();
            }
            closed_cans_sum[i + 1] = closed_cans_sum[i];
            continue;
        }

        opener_cnt -= 1;
        let closed_can = closed_cans.pop().unwrap();
        closed_cans_sum[i + 1] = closed_cans_sum[i] + closed_can;
    }

    println!(
        "{}",
        (0..=m)
            .map(|i| opened_cans_sum[i] + closed_cans_sum[m - i])
            .max()
            .unwrap()
    );
}
