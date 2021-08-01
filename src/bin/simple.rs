fn main() {
    let number = 25;
    let x = match number {
        // Match a single value
        1 => "One!",
        // Match several values
        2 | 3 | 5 | 7 | 11 => "This is a prime",
        // Match an inclusive range
        13..=19 => "A teen",
        // Handle the rest of cases
        _ => "Ain't special",
    };
    println!("Output: {}", x);
    println!("Range: {:?}", (1..10).collect::<Vec<u8>>());
}
