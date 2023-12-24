use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    };

    let mut logo_max = 0;
    let mut logo_sum = 0;
    let mut m_stock = m;
    for d in 0..n {
        if s[d] == '1' {
            if m_stock == 0 {
                logo_sum += 1;
                logo_max = logo_max.max(logo_sum);
            } else {
                m_stock -= 1;
            }
        } else if s[d] == '2' {
            logo_sum += 1;
            logo_max = logo_max.max(logo_sum);
        } else {
            m_stock = m;
            logo_sum = 0;
        }
    }
    println!("{logo_max}");
}
