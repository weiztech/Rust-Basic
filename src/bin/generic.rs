struct Person<T, U> {
    name: T,
    age: U,
}

// Generic  Trait

impl<T, U> Person<T, U> {
    fn new(name: T, age: U) -> Self {
        Person { name, age }
    }
}

trait Mix<T, U> {
    fn combine<V, W>(self, other: Person<V, W>) -> Person<T, W>;
    fn call_me(&self) -> &T;
}

impl<T, U> Mix<T, U> for Person<T, U> {
    fn combine<V, W>(self, other: Person<V, W>) -> Person<T, W> {
        Person {
            name: self.name,
            age: other.age,
        }
    }

    fn call_me(&self) -> &T {
        &self.name
    }
}

// Trait as Parameters
fn show_name<T, U>(people: &impl Mix<T, U>) -> &T {
    people.call_me()
}

// Trait bounds
fn show_name_2<T, X, U>(people: &T) -> &X
where
    T: Mix<X, U>,
{
    people.call_me()
}

fn main() {
    let p1 = Person::new("Agni", 10);
    let p2 = Person {
        name: 'A',
        age: 20.5,
    };
    println!("p1 {} age {}", p1.name, p1.age);
    println!("p2 {} age {}", p2.name, p2.age);

    let pmix = p1.combine(p2);
    println!("pmix {} age {}", pmix.name, pmix.age);

    println!("show_name {}", show_name(&pmix));
    println!("show_name_2 {}", show_name_2(&pmix));
}
