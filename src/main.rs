mod shadowing;
mod mutability;
mod data_types;
mod control_flow;
mod loops;


fn main() {
    //Mutability
    println!("Mutability\n");
    mutability::mutability();

    println!("=======================");
    println!("Shadowing\n");
    shadowing::shadowing();

    println!("=======================");
    println!("Data Types\n");
    data_types::datatypes();

}
