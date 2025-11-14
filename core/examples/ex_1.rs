use nation_of_last_land_core::Core;

fn main() {
    let mut core = Core::new();

    for _i in 0..100 {
        core.update(15.0).unwrap();
        let s = core.export_world();
        println!("Export world: {s}");
    }

    println!("Hello, world!");
}
