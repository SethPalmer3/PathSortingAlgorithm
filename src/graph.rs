pub mod cartesian;
pub trait Point{
    type Index;
    fn new(i: Self::Index) -> Self;
    fn dist(&self, o: &Self) -> Option<f64>;
}

pub trait Graph {
    type I;
    type P;
    fn new() -> Self;
    fn add(&mut self, p: Self::P);
    fn get_element(&self, i: Self::I) -> Option<&Self::P>;
    fn path_dist(&self, v: &Vec<Self::I>) -> Option<f64>;
    fn swap(&mut self, a: Self::I, b: Self::I);
}
