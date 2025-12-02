use nation_of_last_land_core::Core;

fn main() {
    let mut core = Core::new();

    core.update(15.0).unwrap();

    let s = core.export_world(true);
    
    println!("Export world: {s}");

    // for _i in 0..100 {
    //     core.update(15.0).unwrap();
    //     let s = core.export_world();
    //     println!("Export world: {s}");
    // }
}
