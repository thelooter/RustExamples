mod shadowing;
mod mutability;
mod data_types;
mod functions;
mod control_flow;
mod loops;
mod variable_scope;
mod ownership;


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
