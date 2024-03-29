use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Write;
use crate::graph::Point;
use crate::sorters;
use super::super::Graph;
use super::point::CartesianPoint;

#[derive(Debug)]
pub struct CartesianPlane {
    points: Vec<CartesianPoint>,
}

impl CartesianPlane {
    fn get_vec_index(&self, i: <CartesianPlane as Graph>::I) -> Option<usize>{
        for ind in 0..self.points.len(){
            if self.points[ind].index == i{
                return Some(ind);
            }
        }
        return None;
    }
    pub fn calc_min_spanning_tree_weight(&self) -> f64{
        let mut edges = Vec::new();
        for i in 0..self.points.len(){
            for j in 0..self.points.len(){
                if i == j {continue;}
                edges.push( (i, j, self.points[i].dist(&self.points[j]).unwrap_or(f64::MAX)) );
            }
        }
        edges.sort_by(|a,b|{
            (a.2).partial_cmp(&b.2).unwrap()
        });
        let mut points = Vec::new();
        let mut sum = 0.0;
        let mut k = 0;
        while points.len() < self.points.len(){
            let (p1, p2, d) = edges[k];
            let mut added = false;
            if let None = points.iter().position(|&x| x == p1){
                points.push(p1);
                added = true;
            }
            if let None = points.iter().position(|&x| x == p2){
                points.push(p2);
                added = true;
            }
            if added{
                sum += d;
            }
            k+=1;

        }
        return sum;
    }
    pub fn sort_iteration(&self, v: &mut Vec<u32>) {
        let mut current_path_dist;
        for i in 0..v.len()-1{
            current_path_dist = self.path_dist(v).unwrap();
            for k in i+1..v.len(){
                v.swap(i, k);
                if self.path_dist(v).unwrap() < current_path_dist {
                    current_path_dist = self.path_dist(v).unwrap();
                }else{
                    v.swap(i, k); // Undo temp swap
                }
            }
        }

    }
    // pub fn sort_path(&self, v: &mut Vec<u32>, num_fails: u32) {
    //     let mut best_path = v.clone();
    //     let mut count = 0;
    //     let mut current_best_dist;
    //     while count < num_fails{
    //         v.shuffle(&mut thread_rng());
    //         current_best_dist = f64::MAX;
    //         while current_best_dist - self.path_dist(v).unwrap() > 0.0{
    //             current_best_dist = self.path_dist(v).unwrap();
    //             self.sort_iteration(v);
    //         }
    //         if self.path_dist(&best_path).unwrap() > self.path_dist(&v).unwrap(){
    //             best_path = v.clone();
    //             count = 0;
    //             println!("{:?}", self.path_dist(&best_path).unwrap());
    //         }else{
    //             count += 1;
    //         }
    //     }
    //     *v = best_path.clone();
    // }
    pub fn sort_path(&self, v: &mut Vec<u32>, num_fails: u32) {
        let mut best_path = v.clone();
        let mut count = 0;
        let mut current_best_dist;
        while count < num_fails{
            v.shuffle(&mut thread_rng());
            current_best_dist = f64::MAX;
            while current_best_dist - self.path_dist(v).unwrap() > 0.0{
                current_best_dist = self.path_dist(v).unwrap();
                sorters::merge_sort(v, self);
            }
            if self.path_dist(&best_path).unwrap() > self.path_dist(&v).unwrap(){
                best_path = v.clone();
                count = 0;
                println!("{:?}", self.path_dist(&best_path).unwrap());
            }else{
                count += 1;
            }
        }
        *v = best_path.clone();
    }
    pub fn sort_path_by_error(&self, v: &mut Vec<u32>, error: f64, delta: f64) {
        let best = self.calc_min_spanning_tree_weight();
        let mut magnitude = 0.0;
        let mut best_path = v.clone();
        let mut current_best_dist;
        while self.path_dist(&best_path).unwrap() - best > error + (magnitude * delta){
            v.shuffle(&mut thread_rng());
            current_best_dist = f64::MAX;
            while current_best_dist - self.path_dist(v).unwrap() > 0.0{
                current_best_dist = self.path_dist(v).unwrap();
                sorters::merge_sort(v, self);
            }
            if self.path_dist(&best_path).unwrap() > self.path_dist(&v).unwrap(){
                best_path = v.clone();
                magnitude = 0.0;
                println!("{:?}", self.path_dist(&best_path).unwrap());
            }else{
                magnitude += delta;
            }
        }
        *v = best_path.clone();
    }
    pub fn print_points(&self, v: &Vec<u32>){
        for ind in v.iter(){
            let pnt = self.get_element(*ind).unwrap();
            println!("{:?}, {:?}", pnt.x, pnt.y);
        }
    }
    pub fn string_points(&self, v: &Vec<u32>) -> String{
        let mut ret_str = String::from("");
        for ind in v.iter(){
            let pnt = self.get_element(*ind).unwrap();
            write!(ret_str, "{:?}, {:?}\n", pnt.x, pnt.y).unwrap();
        }
        return ret_str;
    }
}
impl Graph for CartesianPlane {
    type I = u32;

    type P = CartesianPoint;

    fn new() -> Self {
        Self { points: vec![] }
    }

    fn add(&mut self, p: Self::P) {
        self.points.push(p)
    }

    fn path_dist(&self, v: &[Self::I]) -> Option<f64> {
        if v.len() > self.points.len() {
            return None;
        }
        let mut sum = 0.0;
        for i in 0..(v.len() - 1) {
            let e1 = self.get_element(v[i])?;
            let e2 = self.get_element(v[i + 1])?;
            sum += e1.dist(e2)?;
        }
        return Some(sum);
    }

    fn swap(&mut self, a: Self::I, b: Self::I) {
        let ind_a = self.get_vec_index(a).unwrap();
        let ind_b = self.get_vec_index(b).unwrap();
        let temp = self.points[ind_a].clone();
        self.points[ind_a] = self.points[ind_b].clone();
        self.points[ind_b] = temp;
    }

    fn get_element(&self, i: Self::I) -> Option<&Self::P> {
        for e in self.points.iter() {
            if e.index == i {
                return Some(e);
            }
        }
        return None;
    }
}
