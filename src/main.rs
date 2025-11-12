use crate::modules::components::Pos;

mod modules;

fn main() {
    let mut core = modules::core::Core::new();

    core.create_vehicle(Pos { x: 10.0, y: 10.0 }).unwrap();

    core.create_waste(Pos { x: 10.0, y: 10.0 }).unwrap();

    println!("Hello, world!");
}
