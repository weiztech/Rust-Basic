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
// re-implement Brightness + OnOff method, for avoid importing multiple traits
pub trait Light: Brightness + OnOff {
    fn set_good(&self) {
        println!("Good light");
    }

    fn set_brightness(&self, brightness: i32) {
        Brightness::set_brightness(self, brightness)
    }

    fn set_onoff(&self, b: bool) {
        OnOff::set_onoff(self, b)
    }
}
impl<T: OnOff + Brightness> Light for T {}
