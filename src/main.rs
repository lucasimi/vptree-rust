mod utils;
mod vptree;

fn main() {
    let mut _v = vec![-4, -2, -3];
    //build(&_v);
    utils::quick_select(&mut _v, 2);
    println!("Hello, world! {:?}", _v);
}
