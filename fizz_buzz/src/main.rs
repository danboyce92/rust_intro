
fn fizz_buzz() {
    let x = 100;
    for number in 1..=x {
        if number % 5 == 0 && number % 3 == 0 {
            println!("FizzBuzz!");
        } else if number % 3 == 0 {
            println!("Fizz!");
        } else if number % 5 == 0 {
            println!("Buzz!");
        } else {
            println!("{}", number);
        }
    }
}

fn main() {
    fizz_buzz();
}
