fn main() {
    let mut res: Vec<u8> = move_zeros(&[1, 0, 1, 2, 0, 1, 3]);
    println!("Hello, world!, {:?}", res);
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    let null:u8 = 0;

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

