//Practice Borrowing, Ownership, Lifecycles rules here
//Bad example.. numbers are not treated the same when it comes to borrowing..

struct Calculate {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Calculate {
    fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Calculate { a, b, c, d }
    }

    fn add(&self, num1: u32, num2: u32) -> u32 {
        num1 + num2
    }

    fn sub(&self, num1: u32, num2: u32) -> u32 {
        num1 - num2
    }

    fn mul(&self, num1: u32, num2: u32) -> u32 {
        num1 * num2
    }

    fn div(&self, num1: u32, num2: u32) -> u32 {
        num1 / num2
    }

    fn print_result(&self, mess: &String, num1: u32, num2: u32) {
        let num3 = num1 + num2;
        println!("{} {}", mess, num3);
    }
}




fn main() {
    let calculator = Calculate::new(12, 53, 82, 21);
    let message: String =  String::from("The following sum equates to : ");
    let x = 30;
    let y = 50;

    //These message bindings do not cause an error because println! only borrows a reference.
    println!("{} {}", message, calculator.add(calculator.a, calculator.b));

    println!("{} {}", message, calculator.mul(calculator.b, calculator.d));

    //Now try with the printfn()
    //Because message is being moved it needs to be a reference or it will cause an error.
    calculator.print_result(&message, x, y);

    println!("{} {}", message, calculator.div(calculator.c, calculator.a));

    println!("{} {}", message, calculator.sub(calculator.d, calculator.a));
}
