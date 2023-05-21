use crate::{
    core::comparators::calculate_largest,
    core::operations_on_generics::{div, mul, sub, sum},
    core::point::Point,
    core::users::{do_job, Administrator, Customer, User},
};

mod core;

fn main() {
    generic_function_comparators();
    generic_function_operations();
    generic_struct_user();
    generic_struct_operations();
}

fn generic_struct_operations() {
    let point: Point<i32> = Point { x: 1, y: 2 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());

    let point2 = Point { x: 1, y: 2 };
    let point1 = Point { x: 3, y: 5 };
    println!("{:?}", point1 + point2);

    let point2 = Point { x: 1.1, y: 2.3 };
    let point1 = Point { x: 3.1, y: 5.3 };
    println!("{:?}", point1 + point2);

    let point2 = Point { x: 1.1, y: 2.3 };
    let point1 = Point { x: 3.1, y: 5.3 };
    println!("{:?}", point1 - point2);

    let point2 = Point { x: 1.1, y: 2.3 };
    let point1 = Point { x: 3.1, y: 5.3 };
    println!("{:?}", point2 / point1);

    let point2 = Point { x: 1.1, y: 2.3 };
    let point1 = Point { x: 3.1, y: 5.3 };
    println!("{:?}", point2 * point1);
}

fn generic_function_operations() {
    println!("1+2 = {}", sum(1, 2));
    println!("1.1+2.2 = {}", sum(1.1, 2.2));

    println!("1*2 = {}", mul(1, 2));
    println!("1.1*2.2 = {}", mul(1.1, 2.2));

    println!("1/2 = {}", div(1, 2));
    println!("1.1/2.2 = {}", div(1.1, 2.2));

    println!("1-2 = {}", sub(1, 2));
    println!("1.1-2.2 = {}", sub(1.1, 2.2));
}

fn generic_struct_user() {
    let administrator = Administrator::new("admin");
    let customer = Customer::new("customer");
    do_job(administrator);
    do_job(customer);
}

fn generic_function_comparators() {
    let numbers = vec![1, 3, 5, 2, 4];
    let characters = vec!['a', 'c', 'e', 'b', 'd'];
    println!("{:?}", calculate_largest::<i32>(&numbers));
    println!("{:?}", calculate_largest::<char>(&characters));
    let void = vec![];
    println!("{:?}", calculate_largest::<char>(&void));
}
