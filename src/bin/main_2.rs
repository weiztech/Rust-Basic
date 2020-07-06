#[path = "../modules/math.rs"]
mod libs;

fn main() {
    let x = 5;
    let x = x * 2;
    libs::sum_no_return(x, x);
}
