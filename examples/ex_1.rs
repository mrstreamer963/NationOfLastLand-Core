use NationOfLastLand_Core::{Core, Pos};

fn main() {
    let mut core = Core::new();

    core.create_vehicle(Pos { x: 10.0, y: 10.0 }).unwrap();
    core.create_vehicle(Pos { x: 20.0, y: 20.0 }).unwrap();

    core.create_waste(Pos { x: 30.0, y: 30.0 }).unwrap();
    core.create_waste(Pos { x: 40.0, y: 40.0 }).unwrap();

    for _i in 0..100 {
        core.update(15.0).unwrap();
        let s = core.export_world();
        println!("Export world: {s}");
    }

    println!("Hello, world!");
}
