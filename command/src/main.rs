#[no_std]
#[allow(warnings)]
mod bindings;
use bindings::component::rpn::types::{Engine, Operation};
//use std::fs;

fn main() {
    let calc = Engine::new();
    calc.push_operand(1);
    calc.push_operand(2);
    calc.push_operation(Operation::Add);
    let sum: u32 = calc.execute();

    let string = format!("[SUM] {:?}", sum);

    println!("{}", string);

    // Print to filesystem
    //fs::write("output.txt", string).unwrap();
}