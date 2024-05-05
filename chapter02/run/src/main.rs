#[allow(warnings)]
mod bindings;

use bindings::component::calc::calc::add;

fn main() {
    const A: i32 = 1;
    const B: i32 = 2;
    let result = add(A, B);
    println!("{A} + {B} = {result}");
}
