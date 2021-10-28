#[derive(Debug)]
pub struct MyLight {
    pub state: bool,
}

pub trait OnOff {
    fn set_onoff(&self, b: bool) {
        println!("OnOff Default");
    }
}

pub trait Brightness {
    fn set_brightness(&self, brightness: i32) {
        println!("Brightness Default");
    }
}

impl OnOff for MyLight {}
impl Brightness for MyLight {}

// Extend trait by make sure it's implements the other traits
/*pub trait Light: Brightness + OnOff {
    fn set_good(&self) {
        println!("Good light");
    }
}
impl Light for MyLight {}*/

// Extend trait using Generic Implementation
// implements the Light trait for all types that implement OnOff and Brightness.
//  Even for types that someone created in another crate that uses your crate.
pub trait Light {
    fn set_good(&self) {
        println!("Good light");
    }
}
impl<T: OnOff + Brightness> Light for T {}
