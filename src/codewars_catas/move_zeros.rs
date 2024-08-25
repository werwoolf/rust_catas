pub fn solution(arr: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    let null: u8 = 0;

    for n in 0..arr.len() {
        if arr[n] == 0 {
            res.push(arr[n]);
        }
    }

    for n in arr.iter().rev() {
        if *n != null {
            res.insert(0, *n);
        }
    }

    return res;
}
