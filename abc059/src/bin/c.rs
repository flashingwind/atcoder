use proconio::input;

fn main() {
    input! {
        n: usize,
        mut arr: [i128;n],
    };
    let mut sum = 0;
    let mut even_minus_cost = 0;
    let mut odd_minus_cost = 0;
    for (i, _) in arr.to_owned().iter().enumerate() {
        if i % 2 == 0 {
            match arr[i] < 0 {
                // 偶数がマイナス
                true => even_minus_cost += arr[i].abs(),
                // 奇数がマイナス
                false => odd_minus_cost += arr[i].abs(),
            }
        } else {
            match arr[i] < 0 {
                // 奇数がマイナス
                true => odd_minus_cost += arr[i].abs(),
                // 偶数がマイナス
                false => even_minus_cost += arr[i].abs(),
            }
        }
    }
    let is_even_minus = even_minus_cost < odd_minus_cost;
    let mut cnt = 0;
    for (i, _) in arr.to_owned().iter().enumerate() {
        // println!(
        //     "{cnt}:      {:?} sum{i}-1={sum} sum{i}={}",
        //     arr,
        //     sum + arr[i]
        // );
        if num::signum(sum) == num::signum(sum + arr[i]) {
            let diff = if is_even_minus && i % 2 == 0 {
                (sum + arr[i]).abs() + 1
            } else {
                -((sum + arr[i]).abs() + 1)
            };
            arr[i] += diff;
            cnt += diff.abs();
            // println!(
            //     "{cnt}: mod1 {:?} sum{i}-1={sum} sum{i}={} {}",
            //     arr,
            //     sum + arr[i],
            //     if is_even_minus && i % 2 == 0 {
            //         (sum + arr[i]).abs() + 1
            //     } else {
            //         -(sum + arr[i]).abs() - 1
            //     }
            // );
        }
        sum += arr[i];
        if sum == 0 {
            sum += if is_even_minus && i % 2 == 0 { -1 } else { 1 };
            arr[i] += if is_even_minus && i % 2 == 0 { -1 } else { 1 };
            cnt += 1;
            // println!(
            //     "{cnt}: mod2 {:?} sum{i}={sum} {}",
            //     arr,
            //     if is_even_minus && i % 2 == 0 { -1 } else { 1 }
            // );
        }
    }
    println!("{}", cnt);
}
