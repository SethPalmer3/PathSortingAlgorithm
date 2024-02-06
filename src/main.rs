use rand::{seq::SliceRandom, thread_rng, Rng};
// use plotters::prelude::*;
use graph::{Graph, Point};
use graph::cartesian::{graph::CartesianPlane, point::CartesianPoint};
pub mod graph;

fn main() {
    let mut cg = CartesianPlane::new();
    let mut rng = rand::thread_rng();
    let pnt_size = 20;
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
    cg.sort_path(&mut path, pnt_size*pnt_size);
    println!("{:?}, {:?}", path, cg.path_dist(&path));
    cg.print_points(&path);

}
