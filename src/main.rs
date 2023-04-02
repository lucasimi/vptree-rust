mod utils;
mod vptree;

fn dist(x: &i32, y: &i32) -> f32 {
    (x - y).abs() as f32
}

fn main() {
    let mut vec = vec![-4, -2, -3];
    let vptree = vptree::build(&vec, dist);
}
