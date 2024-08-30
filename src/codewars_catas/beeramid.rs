pub fn beeramid(bonus: i32, price: f32) -> usize {
    let mut cans = (bonus as f32 / price) as usize;

    if cans < 1 {
        return 0;
    }

    let mut floor = 1;

    loop {
        let next_floor_cans = floor * floor;

        if next_floor_cans > cans {
            break;
        }

        floor += 1;
        cans -= next_floor_cans;
    }

    floor -1
}