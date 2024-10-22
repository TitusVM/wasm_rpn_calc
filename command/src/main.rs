#[allow(warnings)]
mod bindings;
use bindings::component::rpn::types::{Engine, Operation};

fn main() {
    let calc = Engine::new();
    calc.push_operand(1);
    calc.push_operand(2);
    calc.push_operation(Operation::Add);
    let sum = calc.execute();
    println!("{sum}");
}