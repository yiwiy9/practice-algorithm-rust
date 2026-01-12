use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut a_odd: Vec<usize> = a.iter().filter(|&a_i| a_i % 2 == 1).copied().collect();
    let mut a_even: Vec<usize> = a.iter().filter(|&a_i| a_i % 2 == 0).copied().collect();

    let mut ans: usize = 0;

    if a_odd.len() >= k {
        a_odd.sort_by(|a, b| b.cmp(a));
        ans = ans.max(a_odd.iter().take(k).sum::<usize>());
    }

    if a_even.len() >= k {
        a_even.sort_by(|a, b| b.cmp(a));
        ans = ans.max(a_even.iter().take(k).sum::<usize>());
    }

    println!("{}", ans);
}
