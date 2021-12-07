pub(crate) fn variable_scope() {
    // Variables in Rust are only accessible within their scope.
    // This means that if you try to access a variable from outside of its scope, you will get an error.
    // Scope is determined by curly braces {}.
    //After Variable get out of scope, it will be dropped and the memory will be freed.

    // The following code will not compile:
    {
        let x = 5;
        println!("The value of x is: {}", x);
    }
    println!(x);

    // The following code will compile:
    let x = 5;
    println!("The value of x is: {}", x);

    // Variables can be declared in different scopes.
    // The following code will compile:
    let x = 5;
    {
        let y = 10;
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }
}