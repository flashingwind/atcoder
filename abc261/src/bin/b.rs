use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        tab: [Chars;n],
    };
    for (i, l) in tab.iter().enumerate() {
        for (j, _) in l.iter().enumerate() {
            if tab[i][j] == 'W' && tab[j][i] == 'L'
                || tab[i][j] == 'L' && tab[j][i] == 'W'
                || tab[i][j] == 'D' && tab[j][i] == 'D'
                || i == j && tab[i][j] == '-' && tab[j][i] == '-'
            {
            } else {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}
