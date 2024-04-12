use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut a_evens = a.iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>();
    let mut a_odds = a.iter().filter(|&x| x % 2 == 1).collect::<Vec<_>>();

    a_evens.sort_by(|a, b| b.cmp(a));
    a_odds.sort_by(|a, b| b.cmp(a));

    let mut ans = -1;

    if a_evens.len() >= 2 {
        ans = ans.max(a_evens[0] + a_evens[1]);
    }

    if a_odds.len() >= 2 {
        ans = ans.max(a_odds[0] + a_odds[1]);
    }

    println!("{}", ans);
}
