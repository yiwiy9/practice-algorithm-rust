use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        hw: [(usize, usize); n],
    }

    let mut h_sorted = vec![];
    let mut w_sorted = vec![];
    for (i, &(h_i, w_i)) in hw.iter().enumerate() {
        h_sorted.push((h_i, w_i, i));
        w_sorted.push((w_i, h_i, i));
    }
    h_sorted.sort_by(|a, b| b.cmp(a));
    w_sorted.sort_by(|a, b| b.cmp(a));

    let init = (h, w);
    let mut ans = vec![init; n];

    let mut cur_h = h;
    let mut cur_w = w;

    let mut h_idx = 0;
    let mut w_idx = 0;
    while h_idx < n || w_idx < n {
        let mut changed = false;
        while h_idx < n {
            if h_sorted[h_idx].0 > cur_h {
                h_idx += 1;
            } else if h_sorted[h_idx].0 == cur_h {
                let (h_i, w_i, i) = h_sorted[h_idx];
                if ans[i] != init {
                    h_idx += 1;
                    continue;
                }
                ans[i] = (cur_h - h_i + 1, cur_w - w_i + 1);
                cur_w -= w_i;
                h_idx += 1;
                changed = true;
            } else {
                break;
            }
        }

        while w_idx < n {
            if w_sorted[w_idx].0 > cur_w {
                w_idx += 1;
            } else if w_sorted[w_idx].0 == cur_w {
                let (w_i, h_i, i) = w_sorted[w_idx];
                if ans[i] != init {
                    w_idx += 1;
                    continue;
                }
                ans[i] = (cur_h - h_i + 1, cur_w - w_i + 1);
                cur_h -= h_i;
                w_idx += 1;
                changed = true;
            } else {
                break;
            }
        }
        if !changed {
            break;
        }
    }

    for &(x, y) in &ans {
        println!("{} {}", x, y);
    }
}
