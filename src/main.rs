fn main() {

    let number1 = 25;
    println!("The value of number1 is: {}", number1);

    let number2: i32 = 100;
    print!("The value of number2 is {}\n", number2);

    print_sum(20, 50);

}

fn print_sum(x: i32, y: i32) {
    print!("the sum of the two values provided is: {}", x + y);
}
