fn main() {
    println!("=== Fiz Buzz ===");

    for input_number in 1..30 {
        if input_number % 3 == 0 && input_number % 5 == 0 {
            println!("Fizz Buzz");
        } else if input_number % 3 == 0 {
            println!("Fizz");
        } else if input_number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", input_number);
        }
    }
}
