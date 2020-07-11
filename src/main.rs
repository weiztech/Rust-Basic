mod modules;
use modules::math::sum_return;

use std::io;

fn sum_no_return(a: i8, b: i8) {
    println!("My SUM here {sum}", sum = a + b)
}

/*fn sum_return(a: i8, b: i8) -> i8 {
    a + b
}*/

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    let iter = bytes.iter();
    println!(
        "Bytes {:?} {}",
        bytes,
        std::str::from_utf8(&bytes[0..5]).unwrap().to_string()
    );
    for (i, &item) in iter.enumerate() {
        println!("{} {} {}", i, item, item == b' ');
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_loop() {
    let mut language = String::new();
    loop {
        println!("What's this programming language?");
        language.clear();
        io::stdin()
            .read_line(&mut language)
            .expect("Readline Failed");

        println!("{}", language.to_lowercase().trim());
        if language.to_lowercase().trim() == "rust" {
            break;
        }
    }

    println!("Correct answers");
}

fn while_loop() {
    let mut language = String::new();

    while language.trim().to_lowercase() != "rust" {
        language.clear();

        println!("What's this programming language?");
        io::stdin()
            .read_line(&mut language)
            .expect("Readline Failed");

        println!("Line Data: {}", language.to_lowercase().trim());
    }
    println!("Correct answers");
}

fn main() {
    let name = "Yihaa";
    let big_name = format!("Emperor {name}", name = name);
    let maxx = format!("{a} {c} {b}", a = "a", b = 'b', c = 3);
    let mut x: i8 = 9;
    let xchar = 'c';
    x += 1;

    // Tuple
    let mytuple = ('H', "Hiho", 7);
    // Destructuring
    let (f, s, t) = mytuple;

    // List/Arrays
    // arrays cannot have values added or removed at runtime
    let mut mylist = [1, 2, 3];
    // Destructuring
    let [one, two, three] = mylist;
    println!("List {} {} {}", one, two, three);
    mylist[1] = 5;
    let iterator = mylist.iter();
    for data in iterator.enumerate() {
        println!("For X list {:?}", data);
    }

    // Vec
    // Changeable Array
    let mut vec: Vec<i8> = vec![1, 2, 3];
    vec.insert(1, 5);

    // Slices
    let slicer = &vec[0..2];

    // output
    println!("Hello, world! {}", big_name);
    println!("{max} {sum} {}", xchar, max = maxx, sum = x);
    println!("Tuple 0 {}, 1 {}", mytuple.0, mytuple.1);
    println!("{} {} {}", f, s, t);
    println!("{:?} {:?} {:?}", mylist, mytuple, slicer);
    // Functions
    println!("Sum no return Output {:?}", sum_no_return(8, 1));
    println!("Sum Return {}", sum_return(5, 17));

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("slice {} {}", word, s);
    // s.clear();
    println!("slice {} {}", word, s);

    // test_loop();
    // while_loop();

    // Result
    let o: Result<i8, &String> = Ok(8);
    let e: Result<i8, &str> = Err("message");

    println!("Result OK {:?} {:?} {:?}", o, o.ok(), o.err());
    println!("Result ERR {:?} {:?} {:?}", e, e.ok(), e.err());

    assert_eq!(o.ok(), Some(8)); // Ok(v) ok = Some(v)
    assert_eq!(e.ok(), None); // Err(v) ok = None

    assert_eq!(o.err(), None); // Ok(v) err = None
    assert_eq!(e.err(), Some("message")); // Err(v) err = Some(v)

    // Option
    let x: Option<&str> = Some("Hello, world!");
    assert_eq!(x.is_some(), true);
    assert_eq!(x.is_none(), false);
}
