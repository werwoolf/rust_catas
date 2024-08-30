use std::collections::HashMap;

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut map: HashMap<&u8, i32> = HashMap::new();
    let mut res: Vec<u8> = vec![];
    for num in lst {
        if map.contains_key(&num) {
            map.insert(num, map.get(num).unwrap() + 1);
        } else {
            map.insert(num, 1);
        }

        if (map.get(num).unwrap()) <= &(n as i32) {
            res.push(*num)
        }
    }

    res
}