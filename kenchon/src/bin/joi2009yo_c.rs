use proconio::input;
const INF: usize = 1 << 60;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut ans: usize = INF;
    for i in 0..n {
        let original = a[i];
        for j in 1..=3 {
            if j == original {
                continue;
            }
            a[i] = j;
            let mut stack = vec![];
            for k in 0..n {
                if stack.is_empty() {
                    stack.push((a[k], 1));
                } else {
                    let (top_a, top_a_cnt) = stack.pop().unwrap();
                    if top_a == a[k] {
                        stack.push((top_a, top_a_cnt + 1));
                        continue;
                    }

                    if top_a_cnt < 4 {
                        stack.push((top_a, top_a_cnt));
                        stack.push((a[k], 1));
                        continue;
                    }

                    if stack.is_empty() {
                        stack.push((a[k], 1));
                    } else {
                        let (top_a, top_a_cnt) = stack.pop().unwrap();
                        if top_a == a[k] {
                            stack.push((top_a, top_a_cnt + 1));
                            continue;
                        }

                        stack.push((top_a, top_a_cnt));
                        stack.push((a[k], 1));
                    }
                }
            }
            let (top_a, top_a_cnt) = stack.pop().unwrap();
            if top_a_cnt < 4 {
                stack.push((top_a, top_a_cnt));
            }
            ans = ans.min(stack.iter().map(|(_, cnt)| cnt).sum::<usize>());
        }
        a[i] = original;
    }

    println!("{}", ans);
}
