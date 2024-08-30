// let sum: i64 = n.to_string()
// .chars()
// .map( | ch| ch.to_digit(10).unwrap() as i64)
// .sum();
//
// if sum < 10 {
// sum
// } else {
// digital_root(sum)
// }
// }

fn digital_root(n: i64) -> i64 {
    if n == 0 {
        0
    } else {
        1 + (n - 1) % 9
    }
}