#[path = "../modules/math.rs"]
mod libs;
use libs::sum_return;

fn main() {
    let x = 5;
    let x = x * 2;
    println!("{}", sum_return(x, x));

    // Dereference
    let mut x = 5;
    let y = &mut x;
    *y += 10;
    println!("y {}", y)
}
