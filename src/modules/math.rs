pub fn sum_no_return(a: i8, b: i8) {
    println!("My SUM here {sum}", sum = a + b)
}

pub fn sum_return(a: i8, b: i8) -> i8 {
    a + b
}

pub mod greetings {
    // ⭐️ The module has to be public to access from outside
    pub fn hello() {
        println!("Hello, world!");
    }
}
