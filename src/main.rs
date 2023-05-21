use crate::users::{Administrator, Customer, User};
use std::{
    ops::{Add, Div, Mul, Sub},
    process::Output,
};
mod users;
fn main() {
    let numbers = vec![1, 3, 5, 2, 4];
    let characters = vec!['a', 'c', 'e', 'b', 'd'];
    println!("{:?}", calculate_largest::<i32>(&numbers));
    println!("{:?}", calculate_largest::<char>(&characters));
    let void = vec![];
    println!("{:?}", calculate_largest::<char>(&void));

    let point: Point<i32> = Point { x: 1, y: 2 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());

    let administrator = Administrator::new("admin");
    let customer = Customer::new("customer");
    do_job(administrator);
    do_job(customer);

    println!("{}", sum(1, 2));
    println!("{}", sum(1.1, 2.2));

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

fn do_job<T: User>(user: T) -> () {
    println!("{:?}", user.get_username());
    user.do_operation();
}

fn calculate_largest<T>(list: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if list.is_empty() {
        return None;
    }
    let mut largest = &list[0];
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    return Some(largest.to_owned());
}

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
