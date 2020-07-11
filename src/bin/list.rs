// Rust List: Array, Tuple, Range, Vec

fn main() {
    // Array
    let x = [1, 2, 3, 4, 5];
    // Find Index from array value
    println!(
        "5 index is {:?}",
        x.iter().position(|val| val == &5).unwrap()
    );

    println!("{:?} {}", x.iter().sum::<i8>(), x.len());
    // initialize 500 element of array with value 5
    let ys: [i32; 500] = [5; 500];
    println!("{:?} {}", ys.iter().sum::<i32>(), ys.len());
    // Indexing
    println!("YS: {}", ys[200]);
    // Slicing
    let slice: &[i8] = &x[0..2];
    println!("Slice {:?} {}", slice, slice.iter().sum::<i8>());
    // Loop
    // Loop - Array
    for data in &x {
        println!("{:?}", data);
    }
    for data in x.iter().enumerate() {
        println!("{:?}", data);
    }
    // Loop - Slicing
    for data in slice.iter() {
        println!("Loop Slice {}", data);
    }
    // Destruct
    let [first, two] = ["aaa", "bbb"];
    println!("Array {} {}", first, two);

    // Tuple
    let data = (1, 2, "Hehehe");
    println!("Tuple {:?} {:#?}", data, data);
    let tuple_size = format!("{:?}", data);
    match tuple_size.contains(", ") {
        true => {
            // Without Turbo Fish (less code to write with adding type in variabel)
            // let vec: Vec<&str> = tuple_size.matches(", ").collect();
            // Using TurboFish
            let vec = tuple_size.matches(", ").collect::<Vec<&str>>();
            println!("Vec: {:?} {}", vec, vec.len());
        }
        _ => println!("Not found"),
    }
    println!("Contains {} {}", tuple_size, tuple_size.contains(", "));
    // Tuple Index
    println!("Tuple Index {}", data.2);
    // Tuple Destruct
    let (first, two) = ("aaa", "bbb");
    println!("Tuple {} {}", first, two);

    // Range
    let range = 0..10;
    let sum = range.clone().sum::<i8>();
    println!("{}", sum);
    for data in range.step_by(2).enumerate() {
        println!("{:?}", data);
    }
}
