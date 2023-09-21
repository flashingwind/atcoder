use proconio::input;

fn main() {
    input! {
        h_init: u32,
        m_init: u32,
    };

    {
        let h = h_init;
        let mut hh = vec![0; 2];
        hh[0] = h / 10;
        hh[1] = h % 10;
        for m in m_init..60 {
            let mut mm = vec![0; 2];
            mm[0] = m / 10;
            mm[1] = m % 10;
            if hh[0] * 10 + mm[0] < 24 && hh[1] * 10 + mm[1] < 60 {
                println!("{} {}", h, m);
                return;
            }
        }
    }
    for h in (h_init + 1)..24 {
        let mut hh = vec![0; 2];
        hh[0] = h / 10;
        hh[1] = h % 10;
        for m in 0..60 {
            let mut mm = vec![0; 2];
            mm[0] = m / 10;
            mm[1] = m % 10;
            if hh[0] * 10 + mm[0] < 24 && hh[1] * 10 + mm[1] < 60 {
                println!("{} {}", h, m);
                return;
            }
        }
    }
    for h in 0..24 {
        let mut hh = vec![0; 2];
        hh[0] = h / 10;
        hh[1] = h % 10;
        for m in 0..60 {
            let mut mm = vec![0; 2];
            mm[0] = m / 10;
            mm[1] = m % 10;
            if hh[0] * 10 + mm[0] < 24 && hh[1] * 10 + mm[1] < 60 {
                println!("{} {}", h, m);
                return;
            }
        }
    }
}
