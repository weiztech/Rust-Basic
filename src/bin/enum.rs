// Enum Basic
#[derive(Debug)]
enum MathType {
    Sum,
    Median,
    Mean,
}

// Enum with value
enum MathBasic {
    Num(i8),
    Addition(i8, i8),
    Substraction(i8, i8, i8),
    Multiply(i8, i8),
}

// Enum with curly brackets
enum MathBasic2 {
    Num { x: i8 },
    Addition { x: i8, y: i8 },
    Substraction { x: i8, y: i8, z: i8 },
    Multiply { x: i8, y: i8 },
}

fn choose_math(mtype: MathType) {
    println!("mathType is {:?}", mtype);
}

// function match with return Nothing or Something
fn proceed_math(math: MathBasic) -> Option<i8> {
    match math {
        MathBasic::Num(num) => return Some(num),
        MathBasic::Addition(x, y) => return Some(x + y),
        MathBasic::Substraction(x, y, z) => {
            if x < y || x < z {
                println!("X should be bigger than y and z");
                return None;
            };
            return Some(x - y - z);
        }
        MathBasic::Multiply(x, y) => return Some(x * y),
    };
}

fn main() {
    let math = MathType::Sum;
    choose_math(math);

    let basic = MathBasic::Substraction(100, 5, 5);

    // inline version match
    let value = match basic {
        MathBasic::Num(num) => num,
        MathBasic::Addition(x, y) => x + y,
        MathBasic::Substraction(x, y, z) => {
            if x < y || x < z {
                println!("X should be bigger than y and z");
                return;
            };
            x - y - z
        }
        MathBasic::Multiply(x, y) => x * y,
    };
    println!("Substract {}", value);

    let basic2 = proceed_math(MathBasic::Substraction(7, 2, 3));
    println!("Substract {:?}", basic2.unwrap());

    let math2 = MathBasic2::Num { x: 1 };
    // inline version match
    let value = match math2 {
        MathBasic2::Num { x } => x,
        MathBasic2::Addition { x, y } => x + y,
        MathBasic2::Substraction { x, y, z } => {
            if x < y || x < z {
                println!("X should be bigger than y and z");
                return;
            };
            x - y - z
        }
        MathBasic2::Multiply { x, y } => x * y,
    };

    println!("Value {}", value)
}
