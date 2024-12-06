#[no_std]
#[allow(warnings)]
mod bindings;
use bindings::component::rpn::types::{Engine, Operation};
//use std::fs;

fn main() {
    let calc = Engine::new();

    const N: u32 = 5000; 
    let mut nth_triangular = 0;

    for i in 1..=N {
        let mut triangular_number = 0;

        for j in 1..=i {
            calc.push_operand(triangular_number);
            calc.push_operand(j);

            calc.push_operation(Operation::Add);

            triangular_number = calc.execute();
        }

        nth_triangular = triangular_number;
    }
    println!("The {}-th triangular number is {}", N, nth_triangular);
}