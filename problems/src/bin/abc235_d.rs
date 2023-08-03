use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let mut dist = vec![-1; 1_000_000];
    let mut que = std::collections::VecDeque::new();
    dist[1] = 0;
    que.push_back(1);
    while let Some(u) = que.pop_front() {
        let mut next_v = vec![u * a];

        let mut num_chars: Vec<char> = u.to_string().chars().collect();
        let end = num_chars.pop().unwrap();
        if !num_chars.is_empty() && end != '0' {
            let mut new_num_chars = vec![end];
            new_num_chars.extend(num_chars.iter());
            next_v.push(chars_to_decimal(new_num_chars, 10));
        }

        for &v in &next_v {
            if v >= 1_000_000 || dist[v] != -1 {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }

    println!("{}", dist[n]);
}

pub fn chars_to_decimal(n: Vec<char>, base: usize) -> usize {
    let mut result = 0;
    let mut x = 1;
    for &c in n.iter().rev() {
        result += (c as usize - '0' as usize) * x;
        x *= base;
    }
    result
}
