use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars;n],
    };
    let pat = vec![
        "###.?????".chars().collect::<Vec<_>>(),
        "###.?????".chars().collect::<Vec<_>>(),
        "###.?????".chars().collect::<Vec<_>>(),
        "....?????".chars().collect::<Vec<_>>(),
        "?????????".chars().collect::<Vec<_>>(),
        "?????....".chars().collect::<Vec<_>>(),
        "?????.###".chars().collect::<Vec<_>>(),
        "?????.###".chars().collect::<Vec<_>>(),
        "?????.###".chars().collect::<Vec<_>>(),
    ];
    let mut ans = Vec::new();
    for y in 0..=n - 9 {
        for x in 0..=m - 9 {
            let mut is_ok = true;
            for yy in 0..9 {
                for xx in 0..9 {
                    if pat[yy][xx] != '?' && s[y + yy][x + xx] != pat[yy][xx] {
                        is_ok = false;
                        break;
                    }
                }
                if !is_ok {
                    break;
                }
            }
            if is_ok {
                ans.push((y + 1, x + 1));
            }
        }
    }
    for a in ans.iter() {
        println!("{} {}", a.0, a.1);
    }
}
