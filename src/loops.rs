pub(crate) fn loops() {
    //Loops are used to repeat Statements
    //You can make a loop that repeats forever by using the loop keyword
    let condition = true;
    loop {
        println!("This will repeat forever");
        if condition {
            break
        };
    }

    //You can break out of a loop by using the break keyword
    loop {
        println!("This will repeat forever");
        break;
    }

    //You can also use continue to skip the rest of the current iteration
    loop {
        println!("This will repeat forever");
        if condition {
            break;
        };
        continue;
    }

    //You can also use a loop label to break out of a specific loop
    //Loop Labels start with a ' sign
    //You can also use a label with continue
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);


    //You can also return values from a loop
    let result = loop {
        println!("This is the loop");
        break 10;
    };

    println!("The result is {}", result);

    //You can also use a while loop
    //The while loop will run until the condition is false
    //The condition is evaluated before the loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");


    //If you want to loop over a collection you can use the for loop
    //The for loop will iterate over each element in the collection
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    //You can also use a for loop with a range
    for number in 1..4 {
        println!("{}!", number);
    }


}


