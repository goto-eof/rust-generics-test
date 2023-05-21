use generic_functions_test::{generic_function_comparators, generic_function_operations};
use generic_structs_test::{generic_struct_operations, generic_struct_user};

mod core;
mod generic_functions_test;
mod generic_structs_test;

fn main() {
    generic_function_comparators();
    generic_function_operations();
    generic_struct_user();
    generic_struct_operations();
}
