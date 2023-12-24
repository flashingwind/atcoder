use proconio::input;

fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    };
    let mut suml = 0;
    let mut aa = a % m;
    if aa < 0 {
        aa += m;
    } else if aa == 0 {
        suml = 1;
    }
    if 0 <= l {
        if aa <= l {
            suml += (l - aa) / m + 1;
        } else {
            suml += l / m;
        }
    } else {
        aa -= m;
        if l <= aa {
            suml += (aa - l) / m + 1;
        } else {
            suml += l / m;
        }
    }
    // println!("aa={aa}");
    // println!("suml={suml}");
    let mut aa = a % m;
    let mut sumr = 0;
    if aa < 0 {
        aa += m;
    } else if aa == 0 {
        suml = 1;
    }
    // println!("aa={aa}");
    if 0 < r {
        if aa <= r {
            sumr += (r - aa) / m + 1;
        } else {
            sumr += 0;
        }
    } else {
        aa -= m;
        // println!("aa={aa}");
        if r <= aa {
            sumr += 0;
        } else {
            sumr += (aa - r) / m + 1;
        }
    }
    // println!("sumr={sumr}");
    if r == 0 && l == 0 {
        println!("0");
    } else if r <= 0 && l <= 0 {
        // println!("suml - sumr=");
        println!("{}", suml - sumr);
    } else if r >= 0 && l >= 0 {
        // println!("sumr - suml=");
        println!("{}", sumr - suml);
    } else {
        // println!("sumr + suml=");
        println!("{}", sumr + suml);
    }
}
