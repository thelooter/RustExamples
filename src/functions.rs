pub(crate) fn functions() {
    //Functions are declared with the `fn` keyword.
    //Functions have a body, which is made up of statements.
    //The body is wrapped in braces `{}`.
    //The return type is optional.
}

//Functions can have parameters.
fn function_with_parameters(x: i32, y: i32) {
    //The parameters are defined using the `(` and `)` delimiters.
    //The Type of a Parameter must be specified after the parameter name.
    //Example: x: i32
    //The parameters can be separated by commas.
    println!("x: {}, y: {}", x, y);
}

//Functions can have a return type.
//The return type must be specified after the `->` symbol.
//Example: fn sum(x: i32, y: i32) -> i32
fn sum(x: i32, y: i32) -> i32 {
   return  x + y
}

//You can also return the Result of the last expression implicitly.
fn sum_implicit(x: i32, y: i32) -> i32 {
    x + y
}

//Using the return value of a function
fn print_sum(x: i32, y: i32) {
    let sum = sum(x, y);
    println!("sum: {}", sum);
}


