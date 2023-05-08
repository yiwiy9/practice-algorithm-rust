use proconio::input;

const MAX: usize = 100_000;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    let mut memo = vec![];
    let mut dist = vec![MAX; MAX];
    let mut cnt = 0;
    loop {
        if cnt == k {
            println!("{}", n);
            return;
        }
        if dist[n] != MAX {
            break;
        }

        dist[n] = cnt;
        memo.push(n);
        let mut next = n;
        while n != 0 {
            next += n % 10;
            n /= 10;
        }
        next %= MAX;

        n = next;
        cnt += 1;
    }

    let r = (k - dist[n]) % (cnt - dist[n]);
    println!("{}", memo[r + dist[n]]);
}
