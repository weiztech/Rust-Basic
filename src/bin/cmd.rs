use std::{io, process};

#[derive(Debug, Default)]
struct ListData {
    data: Vec<String>,
}

impl ListData {
    fn new() -> Self {
        // Basic/Default Init code
        // ListData { data: Vec::new() }

        // since Default Trait is implemented on `derive`
        // Could use `default()` for init
        Default::default()
    }

    fn add(&mut self) {
        let value: String = user_input("Please type value to add:")
            .trim()
            .parse()
            .expect("Add value not found");

        self.data
            .append(&mut value.split_whitespace().map(|s| s.to_string()).collect());
    }

    fn remove_by_name(&mut self) {
        let value: String = user_input("Please type value to remove:")
            .trim()
            .parse()
            .expect("Remove value not found");

        value.split_whitespace().for_each(|x| {
            let string = x.to_string();
            if self.data.contains(&string) {
                self.data.retain(|value| &string != value);
                println!("Deleted value: {}", string);
            } else {
                println!("Not found value: {}", x)
            }
        });
    }
}

fn menu(list: &mut ListData) {
    println!(
        "\n############ List CMD ############\n\
    Action Lists:\n\
    1) Add name\n\
    2) remove name\n\
    3) show list\n\
    4) exit\n\
    "
    );
    let input: usize = match user_input("Please select your action:").trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Action not found ({}), system exit...\n", err);
            process::exit(1)
        }
    };

    match input {
        1 => list.add(),
        2 => list.remove_by_name(),
        3 => println!("\nshow list: {:?}", list.data),
        4 => process::exit(1),
        _ => {
            println!("\nAction not found, system exit...");
            process::exit(1)
        }
    }
}

fn user_input(message: &'static str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Message not found");
    input
}

fn main() {
    let mut data = ListData::new();
    loop {
        menu(&mut data);
    }
}
