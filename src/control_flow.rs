pub(crate) fn control_flow() {

    //There are several control flow statements in Rust.
    //The most common is the if statement.
    //The if statement is used to execute code based on a condition.
    //The condition is usually a boolean expression.
    //The if statement can have an else clause.
    //The else clause is executed if the condition is false.

    let number = 3;

    if number == 3 {
        println!("number is three!");
    } else {
        println!("number is not three");
    };


    //You can check for more options with else if.
    //You can have as many else if statements as you want.
    //The else if statement is executed if the previous condition is false.
    if number == 3 {
        println!("number is three!");
    } else if number == 4 {
        println!("number is four!");
    } else {
        println!("number is not three or four :(");
    };

    //You can also use if within a let statement.
    //All branches of the if statement need to be of the same type.
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };




}