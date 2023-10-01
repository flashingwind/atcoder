use proconio::input;

fn main() {
    input! {
        a: u32, //無脂乳固形分は
        b: u32,// 乳脂肪分は

    };
    println!(
        "{}",
        if 15 <= a + b && 8 <= b {
            // 乳固形分が15 パーセント以上、乳脂肪分が 8 パーセント以上含まれるものを「アイスクリーム」とする。
            1
        } else if 10 <= a + b && 3 <= b {
            // 上に当てはまらず、乳固形分が 10 パーセント以上、乳脂肪分が 3 パーセント以上含まれるものを「アイスミルク」とする。
            2
        } else if 3 <= a + b {
            // 上のいずれにも当てはまらず、乳固形分が 3 パーセント以上含まれるものを「ラクトアイス」とする。
            3
        } else {
            // 上のいずれにも当てはまらないものを「氷菓」とする。
            4
        }
    );
}
