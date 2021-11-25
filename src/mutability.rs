pub(crate) fn mutability() {

    // By default Variables in Rust are immutable. That means, once a variable is assigned a value, it can't be changed.
    // That helps to avoid bugs
    let x = 5;
    println!("The value of x is: {}", x);

    //To change the value of a variable, we need to use the mut keyword
    // This is called mutability. Now you can change the value of y
    let mut y = 5;

    //Constant variables are immutable by default
    //Trying to reassign a constant variable will result in an error (E0070)
    //The Names of constants are usually written in all caps (SCREAMING_SNAKE_CASE)
    const z: i32 = 5;
    println!("The value of the constant z is {}", z);







}