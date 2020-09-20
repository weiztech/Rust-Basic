struct Planet {
    name: String,
    id: i8,
    has_life: bool,
    size: i8,
}

impl Planet {
    // associated function (classmethod)
    fn create_pluto(id: i8) -> Planet {
        Planet {
            name: String::from("Pluto"),
            has_life: false,
            id,
            size: 10,
        }
    }
    // function (method)
    fn change_size(&mut self, new_size: i8) {
        self.size = new_size;
    }
}

trait Printer {
    fn show_name(&self) -> &String;
}

impl Printer for Planet {
    fn show_name(&self) -> &String {
        &self.name
    }
}

// Tuple Struct
struct Person(String, i8);

// Unit Struct
struct Gender;

fn main() {
    let mut earth = Planet {
        name: String::from("Earth"),
        id: 3,
        has_life: true,
        size: 1,
    };

    println!("Planet {} , Size {}", earth.show_name(), earth.size);

    earth.change_size(10);
    println!(
        "Planet {} size Changes to {},",
        earth.show_name(),
        earth.size
    );

    let pluto = Planet::create_pluto(5);
    println!(
        "New Planet {} , Size {}, Id {}, has life {}",
        pluto.name, pluto.size, pluto.id, pluto.has_life
    );

    // Destruct struct partially
    let Planet { name, .. } = pluto;
    println!("{}", name);
    // Create new Planet using `..` update syntax (work like `spread` operator in js)
    let planet_destruct = Planet {
        name: String::from("Mars"),
        ..pluto
    };
    println!(
        "Planet destruct {} {} {} {}",
        planet_destruct.name, planet_destruct.id, planet_destruct.size, planet_destruct.has_life
    );

    let person = Person(String::from("Max"), 20);
    println!("Person: name {}, age {}", person.0, person.1);
}
