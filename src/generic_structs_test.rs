use crate::{
    core::point::Point,
    core::users::{do_job, Administrator, Customer, User},
};
pub fn generic_struct_operations() {
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

pub fn generic_struct_user() {
    let administrator = Administrator::new("admin");
    let customer = Customer::new("customer");
    do_job(administrator);
    do_job(customer);
}
