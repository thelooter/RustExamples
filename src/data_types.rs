pub(crate) fn datatypes() {

    //There are two types of Data Types in Rust: Scalar and Compound.

    //Scalar Data Types:
    // A scalar data type is a data type that has a single value.
    // Rust has the following scalar data types:
    // - bool
    // - char
    // - i8, i16, i32, i64, i128
    // - u8, u16, u32, u64, u128
    // - f32, f64
    // - usize, isize

    //The default integer type in RUst is i32.
    let x: i128 = 10;

    //The default floating point type in Rust is f64.
    let y: f64 = 3.135;

    //Boolean can be either true or false.
    let z: bool = true;

    // Character Data Type:
    // A character data type is a data type that has a single character.
    // A Character is declared using the character literal syntax.
    // You need to use single quotes for character literals.
    let a: char = 'a';

    //============================================================

    //Compound Data Types:
    // A compound data type is a data type that has multiple values.
    // Rust has the following compound data types:
    // - tuples
    // - arrays

    //Tuples:
    // A tuple is a compound data type that has multiple values.
    // A tuple is declared using the tuple syntax.
    // You need to use parenthesis to declare a tuple.
    //Once the size of the tuple is declared, you can't change it.
    //You can add Datatypes to the tuple using the comma operator.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //You can deconstruct a tuple using the following syntax:
    let (x, y, z) = tup;

    //You can also access the values of the tuple using the following syntax:
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    //Arrays:
    // An array is a compound data type that has multiple values.
    // Unlike in other languages, the size of the array is fixed.
    // Unlike tuples, arrays can only store a single type of data.
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5];

    //You can also create an array using the following syntax:
    let arr3 = [3; 5];
    //The above array will be [3, 3, 3, 3, 3]

    //You can access the values of the array using the following syntax:
    //Try to access the value at an index that doesn't exist will cause an error (index out of bounds)
    let first = arr[0];
    let second = arr[1];
    let third = arr[2];










}