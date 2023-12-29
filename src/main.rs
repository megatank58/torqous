use runtime::Runtime;
use structs::{
    object::{Object, ShapeType::Cube},
    vector::Vector,
};

mod runtime;
mod structs;

fn main() {
    let mut runtime = Runtime::new();

    runtime.set_object("A", Object::new(10, Cube));

    runtime = runtime.run_for(10, |mut runtime| {
        let mut a = *runtime.get_object("A").unwrap();
        a.force(Vector::from(50, 0));
        runtime.set_object("A", a);

        runtime
    });

    println!("{runtime:?}");
}
