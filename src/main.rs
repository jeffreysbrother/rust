fn main() {

    let number1 = 25;
    println!("The value of number1 is: {}", number1);

    let number2: i32 = 100;
    print!("The value of number2 is {}\n", number2);

    print_sum(20, 50);


    // primitive types #############################

    //bool
    let ted: bool = true;
    print!("the value of ted is: {}\n", ted);

    //char...these are created with single ticks ''
    let david = 'x';
    print!("the value of david is: {}\n", david);

    //a lot of numeric types I won't list here

    //arrays
    let aaron = [1, 2, 3];
    let anna = [4, 5, 6, 12, 56, 123];

    print!("aaron has {} elements\n", aaron.len());
    print!("anna has {} elements\n", anna.len());


    // if statements #############################

    let xyz = 999.1;
    if xyz == 13.33 {
        print!("THE VARIABLE HAS A VALUE OF 13.33!!\n");
    } else if xyz == 15.619 {
        print!("THE VARIABLE HAS A VALUE OF 15.619!!\n");
    } else {
        print!("Aww shit. It's just not happening today\n");
    }


    // mutability #############################

    let mut james = 5;
    james = 17;
    print!("The value of the james variable is: {}\n", james);

}

fn print_sum(x: i32, y: i32) {
    print!("the sum of the two values provided is: {}\n", x + y);
}
