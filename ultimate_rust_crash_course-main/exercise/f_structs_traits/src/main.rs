// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.

// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).

// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("{:?}", carrot);
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    grapes.bite();
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    fn bunny_nibbles<T: Bite>(biteble: &mut T) {
        biteble.bite();
    }
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

trait Bite {
    fn bite(self: &mut Self) -> ();
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}

impl Bite for Grapes {
    fn bite(self: &mut Self) -> () {
        self.amount_left -= 1;
    }
}
