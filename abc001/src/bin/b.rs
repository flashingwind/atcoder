use proconio::input;

fn main() {
    input! {
        m: u32,
    };
    let mut vv;
    if m < 100 {
        // 0km： VVの値は00 とする。
        vv = "00".to_string()
    } else if 100 <= m && m <= 5000 {
        // 0.1km 以上 5km 以下：距離 (km) を 10 倍した値とする。1 桁の場合は上位に 0 を付す。
        vv = format!("{:0>2}", (m * 10) / 1000);
    } else if 6000 <= m && m <= 30000 {
        // 6km 以上 30km 以下：距離 (km) に 50 を足した値とする。
        vv = format!("{}", (m + 50000) / 1000);
    } else if 35000 <= m && m <= 70000 {
        // 35km 以上 70km 以下：距離 (km) から 30 を引いて 5 で割った後、 80 を足した値とする。
        vv = format!("{}", ((m - 30000) / 5 + 80000) / 1000);
    } else {
        // 70km より大きい：VVの値は 89 とする。
        vv = "89".to_string()
    }
    println!("{vv}");
}
