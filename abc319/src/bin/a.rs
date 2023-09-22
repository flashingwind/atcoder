use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut players: HashMap<&str, i32> = HashMap::new();
    players.extend(vec![
        ("tourist", 3858),
        ("ksun48", 3679),
        ("Benq", 3658),
        ("Um_nik", 3648),
        ("apiad", 3638),
        ("Stonefeang", 3630),
        ("ecnerwala", 3613),
        ("mnbvmar", 3555),
        ("newbiedmy", 3516),
        ("semiexp", 3481),
    ]);
    println!("{}", players[&s.as_str()]);
}
