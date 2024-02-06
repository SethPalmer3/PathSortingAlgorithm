use rand::{seq::SliceRandom, thread_rng, Rng};
use std::fs;
use std::process::Command;
use graph::{Graph, Point};
use graph::cartesian::{graph::CartesianPlane, point::CartesianPoint};
pub mod graph;
pub mod sorters;

fn main() {
    let mut cg = CartesianPlane::new();
    let mut rng = rand::thread_rng();
    let pnt_size = 50;
    (0..pnt_size).for_each(|i| {
        let mut pnt = CartesianPoint::new(i);
        pnt.x = rng.gen_range(-10..=10);
        pnt.y = rng.gen_range(-10..=10);
        println!("Point({:?}): x={:?} y={:?}", pnt.index, pnt.x, pnt.y);
        cg.add(pnt);
    });
    let mut path: Vec<u32> = (0..pnt_size).collect();
    path.shuffle(&mut thread_rng());
    // println!("{:?}, {:?}", path, cg.path_dist(&path));
    println!("theoretical - {:?}", cg.calc_min_spanning_tree_weight());
    cg.sort_path_by_error(&mut path, 20.0, 0.1);
    let _ = fs::write("./data.csv", cg.string_points(&path));
    Command::new("python3").arg("./display_data.py").spawn().expect("failed to execute");

}
