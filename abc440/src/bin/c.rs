use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            w: usize,
            c: [usize; n],
        }

        if w >= n {
            println!("0");
            continue;
        }

        let mut compressed = vec![0; 2 * w];
        for i in 0..2 * w {
            let mut sum = 0;
            let mut j = i;
            while j < n {
                sum += c[j];
                j += 2 * w;
            }
            compressed[i] = sum;
        }

        let mut cur = compressed.iter().take(w).sum::<usize>();
        let mut ans = cur;
        for i in w..3 * w {
            cur += compressed[i % (2 * w)];
            cur -= compressed[(i - w) % (2 * w)];
            ans = ans.min(cur);
        }

        println!("{}", ans);
    }
}
