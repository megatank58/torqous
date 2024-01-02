use runtime::Runtime;
use structs::{
    object::{Object, ShapeType::Circle},
    vector::Vector,
};

mod runtime;
mod structs;

fn main() {
    let mut runtime = Runtime::new();

    let mut block = Object::new(10.0, Circle(7.0));

    block.set_position(Vector::from(0.0, 0.0));
    block.force(Vector::from(51.0, 0.0));

    runtime.set_object("Block", block);

    runtime.s_friction = 0.5;
    runtime.k_friction = 0.5;
    runtime.end_time = 5.0;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Torqous", native_options, Box::new(|_cc| Box::new(runtime))).unwrap();
}
