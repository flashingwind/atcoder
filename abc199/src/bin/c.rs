use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        cs: Chars,
        q: usize,
    };
    let mut is_fliped = false;
    let mut s: Vec<String> = cs.iter().map(|c| c.to_string()).collect();
    // println!("org: {}: {is_fliped}", s.join(""));
    for _t in 0..q {
        input! {
            t: usize,
            mut a: usize,
            mut b: usize,
        };
        // println!("  {:?}", s);
        if t == 1 {
            b -= 1;
            a -= 1;
            // todo: is_flipedにあわせて処理
            (a, b) = flipped(is_fliped, a, b, n);
            // println!("  swap {a}:{},{b}:{}", s[a], s[b]);
            let tmp = s[a].clone();
            s[a] = s[b].clone();
            s[b] = tmp;
            // println!("  swap {a}:{},{b}:{}", s[a], s[b]);
        } else {
            is_fliped = !is_fliped;
        }
        if is_fliped {
            // println!(
            //     "  {_t}: {} {}: {is_fliped}",
            //     s[n..].join(""),
            //     s[..n].join("")
            // );
        } else {
            // println!("  {_t}: {}: {is_fliped}", s.join(""));
        }
    }
    if is_fliped {
        println!("{}{}", s[n..].join(""), s[..n].join(""));
    } else {
        println!("{}", s.join(""));
    }
}

fn flipped(is_flipped: bool, a: usize, b: usize, len: usize) -> (usize, usize) {
    let mut aa = a;
    let mut bb = b;
    if is_flipped {
        if a < len {
            aa += len;
        } else {
            aa -= len;
        }
        if b < len {
            bb += len;
        } else {
            bb -= len;
        }
    }
    return (aa, bb);
}
