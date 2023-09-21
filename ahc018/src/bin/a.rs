use proconio::input;

fn main() {
    // 消費体力がKPI
    input! {
        n: usize,// N行 x N列のグリッド
        num_water: usize,// W箇所に水源
        num_house: usize,// K箇所に家
        c: usize,//体力消費のオフセット(体力C+PでPの掘削ができる)。C in {1,2,4,8,16,32,64,128}
        water: [(usize,usize);num_water],// 水源位置
        house: [(usize,usize);num_house],// 家位置
    };
    // 頑丈さ S(i,j)は非公開
    loop {
        // 掘削実行
        println!("{} {} {}", y, x, P);
        input! {
            res: i32,
        };
        if res == 2 || res == -1 {
            // 完成:2 不正な出力:-1
            return;
        }
        // 未破壊:0 破壊成功:1
    }
}
