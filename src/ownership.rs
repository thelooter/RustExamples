pub(crate) fn ownership() {
    // if we the value of a variable to a different variable, the memory location of the original variable will be used
    // y points to the same memory location as x
    let x = 5;
    let y = x;

    //Strings are stored in the heap
    //In Memory the consist of a pointer to the first character and the length of the string as well as its capacity
    //String literals are stored in the stack, since they are immutable and their length is known at compile time
    //WHen copying a String, you move the String struct to the s2 variable.
    //The s1 variable will be invalid. This is because the String struct is moved to the s2 variable.
    //This prevents double free errors since that could lead to memory corruption, which is a security risk.

    let s1 = String::from("hello");
    let s2 = s1;

    //if we want to copy a String, we can use the clone method instead
    //this will create a new String struct and copy the data from the old String struct

    let s3 = s1.clone();

    //Variables like integers and booleans are stored on the stack and can therefore be copied using the = operator
    //Since it is a copy, the original variable will not be changed
    //There is no shallow and deep copy here
    let x = 5;
    let y = x;

    //Passing a variable to a function will transfer ownership of the variable to the function
    //The variable will be invalidated after the function call
    takes_ownership(s2);

    //We can also give ownership from a function to a variable
    let s4 = gives_ownership();

    //We can also take ownership from a function and return it to the caller
    let s5 = takes_and_gives_back(s4);

    //We can also return multiple values from a function
    let (s6, len) = calculate_length(s5);
    println!("The length of '{}' is {}", s6, len);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}