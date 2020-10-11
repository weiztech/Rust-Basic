#[derive(Debug)]
struct Coin(f32, &'static str);

struct CoffeMachine {
    coffees: Vec<f32>,
    coins: [Coin; 8],
}

fn div_mod(a: f32, b: f32) -> (usize, f32) {
    let decimal = format!("{:.2}", a % b).parse::<f32>().unwrap();
    ((a / b) as usize, decimal)
}

impl CoffeMachine {
    fn create_new(coffee_prices: Option<Vec<f32>>) -> Self {
        CoffeMachine {
            coffees: coffee_prices.unwrap(),
            coins: [
                Coin(2.0, "€2"),
                Coin(1.0, "€1"),
                Coin(0.5, "€0.50"),
                Coin(0.2, "€0.20"),
                Coin(0.1, "€0.10"),
                Coin(0.05, "€0.05"),
                Coin(0.02, "€0.02"),
                Coin(0.01, "€0.01"),
            ],
        }
    }

    fn create_order(&self, money: f32, selected_coffee: usize) -> Vec<(&str, usize)> {
        let coffee_price: f32 = self.coffees[selected_coffee];
        let return_amount = money - coffee_price;
        let returns = self.get_return_money(return_amount);
        println!("{}", return_amount);

        /*println!(
            "\n\n\nCoffe Price: {}\nYour Money: {}\nReturn Amount: {}\nReturn Money: {:?}",
            coffee_price, money, return_amount, returns
        );*/
        returns
    }

    fn get_return_money(&self, return_amount: f32) -> Vec<(&str, usize)> {
        let mut return_money: Vec<(&'static str, usize)> = Vec::new();
        let mut amount: f32 = return_amount;
        for coin in self.coins.iter() {
            if amount >= coin.0 {
                let (counter, remaining) = div_mod(amount, coin.0);
                amount = remaining;
                return_money.push((coin.1, counter))
            }
        }
        return_money
    }
}

fn main() {
    let machine = CoffeMachine::create_new(Some(vec![3.14, 6.2]));
    println!("{:?}", machine.create_order(55.55, 0));
    /*
        Output:

        Coffe Price: 3.14
        Your Money: 55.55
        Return Amount: 52.41
        Return Money: [("€2", 26), ("€0.20", 2), ("€0.01", 1)]
    */
}
