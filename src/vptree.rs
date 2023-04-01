use crate::utils;
use fastrand;

pub trait Metric<T> {

    fn eval(x: T, y: T) -> f32;

}

#[derive(Debug)]
pub struct PointWithDist<'a, T: 'a> {
    data: &'a T,
    dist: f32
}

impl<'a, T: 'a> PartialEq for PointWithDist<'a, T> {

    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }

}

impl<'a, T: 'a> PartialOrd for PointWithDist<'a, T> {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }

}

impl<'a, T: 'a> PointWithDist<'a, T> {

    fn new(x: &'a T) -> PointWithDist<'a, T> {
        PointWithDist { data: x, dist: 0.0f32 }
    }

}



struct VPNode<'a, T: 'a> {

    data: &'a T,
    left: Option<Box<VPNode<'a, T>>>,
    right: Option<Box<VPNode<'a, T>>>

}

fn process_slice<T>(vec: &mut [PointWithDist<T>], metric: fn(&T, &T) -> f32) -> () {
    let pivot: usize = fastrand::usize(0..vec.len());
    vec.swap(pivot, 0);
    let mid: usize = (vec.len() - 1) / 2;
    for i in 0..vec.len() {
        vec[i].dist = metric(vec[0].data, vec[i].data);
    }
    println!("len={:?}, mid={:?}", vec[1..].len(), mid);
    utils::quick_select(&mut vec[1..], mid);
    vec[0].dist = vec[mid].dist;
}

pub fn build<T>(vec: &[T], metric: fn(&T, &T) -> f32) -> Vec<PointWithDist<T>> {
    let mut vp_vec: Vec<PointWithDist<T>> = vec.iter()
        .map(|x| PointWithDist::new(x))
        .collect();
    build_iter(&mut vp_vec, metric);
    vp_vec
}

fn build_iter<T>(vp_vec: &mut [PointWithDist<T>], metric: fn(&T, &T) -> f32) -> () {
    let mut stack: Vec<&mut [PointWithDist<T>]> = Vec::with_capacity(vp_vec.len());
    stack.push(vp_vec);
    while let Some(vec) = stack.pop() {
        process_slice(vec, metric);
        if vec.len() > 1 {
            let mid = (vec.len() - 1) / 2;
            let (left, right) = vec[1..].split_at_mut(mid);
            if !left.is_empty() {
                stack.push(left);
            }
            if !right.is_empty() {
                stack.push(right);
            }
        }
    }
}

pub fn search<'a, T>(vp_vec: &[PointWithDist<'a, T>], metric: fn(&T, &T) -> f32, target: &T, eps: f32) -> Vec<&'a T> {
    let mut results: Vec<&T> = vec![];
    let mut stack: Vec<&[PointWithDist<T>]> = Vec::with_capacity(vp_vec.len());
    stack.push(vp_vec);
    while let Some(vec) = stack.pop() {
        let center = &vec[0];
        let dist = metric(center.data, target);
        let mid = 1 + (vec.len() - 1) / 2;
        if dist <= eps {
            results.push(center.data);
        }
        if dist < center.dist + eps {
            stack.push(&vec[1..mid]);
        } 
        if dist >= center.dist - eps {
            stack.push(&vec[mid..]);
        }
    }
    results
}

mod tests {

    use crate::vptree::build;

    #[test]
    fn test_build_full() {
        let metric: fn(&i32, &i32) -> f32 = |n: &i32, m: &i32| {n - m} as f32;
        let mut _v = vec![-1, 4, -4, 1, -2, -3];
        let vp_vec = build(&mut _v, metric);
        println!("Transformed {:?}", vp_vec);
    }

}
