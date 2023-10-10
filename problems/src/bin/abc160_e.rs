use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut p: [usize; a],
        mut q: [usize; b],
        mut r: [usize; c],
    }

    p.sort_by(|a, b| b.cmp(a));
    q.sort_by(|a, b| b.cmp(a));
    r.sort_by(|a, b| b.cmp(a));

    let mut pq = vec![];
    p.iter().take(x).for_each(|&v| pq.push(v));
    q.iter().take(y).for_each(|&v| pq.push(v));
    pq.sort();

    let mut cur = pq.iter().sum::<usize>();
    let mut ans = cur;
    for (i, r_i) in r.iter().enumerate() {
        if i >= x + y || cur + r_i <= pq[i] {
            break;
        }
        cur += r_i;
        cur -= pq[i];
        ans = ans.max(cur);
    }

    println!("{}", ans);
}
