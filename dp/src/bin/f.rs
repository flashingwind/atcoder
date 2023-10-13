use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut ss: Vec<Vec<Option<Vec<char>>>> = vec![vec![None; t.len()]; s.len()];
    lcs(&s, &t, s.len() - 1, t.len() - 1, &mut ss);
    // println!("ss=={:?}", ss);
    if ss.is_empty() {
    } else {
        if let Some(new_ss) = ss[s.len() - 1][t.len() - 1].clone() {
            let str = new_ss
                .iter()
                .map(|v| v.to_string())
                .fold(String::new(), |bfore, str| bfore + str.as_str());
            println!("{}", str);
        }
    }
}

fn lcs<'a>(
    s: &'a Vec<char>,
    t: &'a Vec<char>,
    x: usize,
    y: usize,
    ss: &'a mut Vec<Vec<Option<Vec<char>>>>,
) -> usize {
    if x == 0 && y == 0 {
        ss[0][0] = Some(vec!['E']);
        return 0;
    }
    // println!("{},{}: {:?}", x, y, ss);
    if let Some(tmp) = &ss[x][y] {
        return tmp.len();
    }

    let mut len = 0;
    let mut ss = ss;
    if 0 < x {
        let new_len = lcs(s, t, x - 1, y, ss);
        if len < new_len {
            ss[x][y] = ss[x][y - 1].clone();
            // ss[x - 1][y] = ss[x][y].clone();
            len = new_len;
        }
    }
    if 0 < y {
        let new_len = lcs(s, t, x, y - 1, ss);
        if len < new_len {
            ss[x][y] = ss[x][y - 1].clone();
            // ss[x][y - 1] = ss[x][y].clone();
            len = new_len;
        }
    }

    if 0 < x && 0 < y && s[x - 1] == t[y - 1] {
        let new_len = lcs(s, t, x - 1, y - 1, ss);
        println!("{x},{y}: ss[x][y]={}", ss[x][y].is_some());
        if len < new_len {
            len = new_len;
            if let Some(new_str) = &ss[x - 1][y - 1] {
                let mut join = new_str.to_owned();
                join.push(s[x - 1]);
                ss[x][y] = Some(join);
            } else {
                ss[x][y] = Some(vec![s[x - 1]]);
            }
        }
    }

    // *ss = ss.clone();
    // ss[x][y] = Some();
    return len;
}
