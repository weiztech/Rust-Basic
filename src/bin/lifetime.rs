fn foo<'a, 'b>(x: &'a u32, _y: &'b u32) -> &'a u32 {
    x
}
struct Foo<'a> {
    x: &'a i32,
}

struct Play<'a> {
    data: Vec<&'a str>,
}

fn main() {
    let x = 12;
    let z: &u32 = {
        let y = 42;
        foo(&x, &y)
    };
    println!("Z {}", z);

    let mut play = Play { data: Vec::new() };
    play.data.append(&mut vec!["asad", "master", "gold"]);
    println!("Play {:?}", play.data);

    let f: Foo;
    {
        let n = 5; // variable that is invalid outside this block
        let y = &n;
        f = Foo { x: y };
        // Print will be fine
        println!("{}", f.x);
    };
    // Print will trigger Error due to `n` out of scope/dropped
    println!("{}", f.x);
}
