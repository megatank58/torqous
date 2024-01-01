use runtime::Runtime;
use structs::{
    object::{Object, ShapeType::Circle},
    vector::Vector,
};

mod runtime;
mod structs;

fn main() {
    let mut runtime = Runtime::new();

    runtime.init_ui();

    let mut block = Object::new(10, Circle(0.0));

    block.set_position(Vector::from(0, 0));
    block.force(Vector::from(51, 0));

    runtime
        .set_object("Block", block)
        .set_s_friction(0.5);

    runtime = runtime.run(10.0);

    println!("{runtime:?}");
}
