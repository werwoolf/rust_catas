const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let _t: i8;
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {ready} of my {missiles} missiles...");

    // missiles -= ready;

    println!("{} left...", missiles - ready)
}
