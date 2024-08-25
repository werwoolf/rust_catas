pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= window || h <= 0.0  { return -1 }
    if bounce >= 1.0 || bounce <= 0.0 { return -1 }

    let mut curr_h = h;
    let mut curr_times_seen = 0;

    while curr_h > window {
        curr_h = curr_h * bounce;
        curr_times_seen += 1;

        if curr_h > window {
            curr_times_seen += 1;
        }
    }

    curr_times_seen
}