use super::super::Point;

#[derive(Debug, Clone)]
pub struct CartesianPoint {
    pub x: i32,
    pub y: i32,
    pub index: u32,
}
impl Point for CartesianPoint {
    type Index = u32;

    fn new(i: Self::Index) -> Self {
        Self {
            x: 0,
            y: 0,
            index: i,
        }
    }

    fn dist(&self, o: &Self) -> Option<f64> {
        Some(((self.x - o.x).pow(2) as f64 + (self.y - o.y).pow(2) as f64).sqrt())
    }
}
