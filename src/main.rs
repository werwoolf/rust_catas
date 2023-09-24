fn main() {
    let example_arr = &[
        vec![1, 0, 2, 0, 2, 1],
        vec![1, 0, 2, 1, 5, 7],
        vec![4, 1, 1, 0, 1, 9]
    ];
    let res = connected_values(example_arr, 2, (0, 2));

    println!("{:?}", res);
}

fn connected_values(arr: &[Vec<u8>], val: u8, coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut response = Vec::new();
    let val_in_coord = arr[coord.0][coord.1];

    let is_matched_value = val == val_in_coord;

    if !is_matched_value {
        return response;
    }

    for row in 0..arr.len() {
        for column in 0..arr[0].len() {
            let is_init_val = row == coord.0 && column == coord.1;
            if is_init_val {
                response.push((row, column));
                continue;
            }

            let is_match_value = arr[row][column] == val;

            if !is_match_value {
                continue;
            };

            let neighboring_coordinates = vec![
                (row as isize - 1, column as isize - 1),
                (row as isize - 1, column as isize),
                (row as isize - 1, column as isize + 1),
                (row as isize, column as isize - 1),
                (row as isize, column as isize + 1),
                (row as isize + 1, column as isize - 1),
                (row as isize + 1, column as isize),
                (row as isize + 1, column as isize + 1),
            ];

            let filtered_neighborhoods = neighboring_coordinates
                .into_iter()
                .filter(|(row, column)| (*row > -1 && *row < arr.len() as isize) && (*column > -1 && *column < arr[1].len() as isize))
                .collect();

            println!("{:?}", filtered_neighborhoods);

            response.push((row, column));
        }
    }

    response
}

