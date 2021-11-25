pub(crate)fn shadowing() {
    // This is a shadowing example.
    // The variable `x` is declared and initialized in the outer scope.
    let x = 5;

    // The variable `x` is shadowed by repeating let x;
    // During that, the memory location of `x` is reused.
    // The name x refers now to the new value.
    let x = x + 1;
    {
        // The variable `x` is shadowed by the inner scope.
        // In this case, the inner scope is the block.
        // This value is not accessible outside the block.
        let x = x * 2;
        println!("The value of x is: {}", x);
    }

    //As we can see , the value of `x` is still 6.
    println!("The value of x is: {}", x);


    //We create a new Variable String
    let string = "hello";

    //By shadowing the variable string, we can change the value of the variable string
    //It is now a number
    //If we try to do this with a mutable variable, we will get an error (E0308)
    let string = string.len();
    println!("The length of the string is: {}", string);
}