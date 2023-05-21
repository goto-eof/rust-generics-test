use crate::{
    core::comparators::calculate_largest,
    core::operations_on_generics::{div, mul, sub, sum},
};
pub fn generic_function_comparators() {
    let numbers = vec![1, 3, 5, 2, 4];
    let characters = vec!['a', 'c', 'e', 'b', 'd'];
    println!("{:?}", calculate_largest::<i32>(&numbers));
    println!("{:?}", calculate_largest::<char>(&characters));
    let void = vec![];
    println!("{:?}", calculate_largest::<char>(&void));
}

pub fn generic_function_operations() {
    println!("1+2 = {}", sum(1, 2));
    println!("1.1+2.2 = {}", sum(1.1, 2.2));

    println!("1*2 = {}", mul(1, 2));
    println!("1.1*2.2 = {}", mul(1.1, 2.2));

    println!("1/2 = {}", div(1, 2));
    println!("1.1/2.2 = {}", div(1.1, 2.2));

    println!("1-2 = {}", sub(1, 2));
    println!("1.1-2.2 = {}", sub(1.1, 2.2));
}
