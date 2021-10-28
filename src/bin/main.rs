#[path = "../modules/math.rs"]
mod libs;
use libs::sum_return;
#[path = "../extend_traits.rs"]
mod extend_traits;
use extend_traits::{Brightness, Light, MyLight, OnOff};

fn main() {
    let x = 5;
    let x = x * 2;
    println!("{}", sum_return(x, x));

    // Dereference
    let mut x = 5;
    let y = &mut x;
    *y += 10;
    println!("y {}", y);

    // Extend traits
    let light = MyLight { state: false };
    light.set_brightness(10);
    light.set_onoff(true);
    light.set_good();
    println!("{:?}", light);
}
