pub fn sumx(a: i8, b: i8) -> i8 {
    a + b
}

pub mod greetings {
    // ⭐️ The module has to be public to access from outside
    pub fn hello() {
        println!("Hello, world!");
    }
}
