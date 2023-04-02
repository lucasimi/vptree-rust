use crate::utils;
use fastrand;

type Scalar = f32;

type Metric<T> = fn(&T, &T) -> Scalar;

#[derive(Debug)]
struct Circle<'a, T: 'a> {
    center: &'a T,
    radius: Scalar
}

impl<'a, T: 'a> PartialEq for Circle<'a, T> {

    fn eq(&self, other: &Self) -> bool {
        self.radius == other.radius
    }

}

impl<'a, T: 'a> PartialOrd for Circle<'a, T> {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.radius.partial_cmp(&other.radius)
    }

}

impl<'a, T: 'a> Circle<'a, T> {

    fn new(x: &'a T) -> Circle<'a, T> {
        Circle { center: x, radius: 0.0f32 }
    }

}

pub struct VPTree<'a, T: 'a> {
    vp_vec: Vec<Circle<'a, T>>,
    metric: Metric<T>
}

struct VPNode<'a, T: 'a> {
    data: &'a T,
    left: Option<Box<VPNode<'a, T>>>,
    right: Option<Box<VPNode<'a, T>>>
}

fn process_slice<T>(vec: &mut [Circle<T>], metric: Metric<T>) -> () {
    let pivot: usize = fastrand::usize(0..vec.len());
    vec.swap(pivot, 0);
    let mid: usize = (vec.len() - 1) / 2;
    for i in 0..vec.len() {
        vec[i].radius = metric(vec[0].center, vec[i].center);
    }
    println!("len={:?}, mid={:?}", vec[1..].len(), mid);
    utils::quick_select(&mut vec[1..], mid);
    vec[0].radius = vec[mid].radius;
}

pub fn build<T>(vec: &[T], metric: Metric<T>) -> VPTree<T> {
    let mut vp_vec: Vec<Circle<T>> = vec.iter()
        .map(|x| Circle::new(x))
        .collect();
    build_iter(&mut vp_vec, metric);
    VPTree { 
        vp_vec: vp_vec,
        metric: metric
    }
}

fn build_iter<T>(vp_vec: &mut [Circle<T>], metric: Metric<T>) -> () {
    let mut stack: Vec<&mut [Circle<T>]> = Vec::with_capacity(vp_vec.len());
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

pub fn search<'a, T: 'a>(vp_vec: VPTree<'a, T>, metric: Metric<T>, target: &T, eps: Scalar) -> Vec<&'a T> {
    let mut results: Vec<&T> = vec![];
    let mut stack: Vec<&[Circle<T>]> = Vec::with_capacity(vp_vec.vp_vec.len());
    stack.push(&vp_vec.vp_vec);
    while let Some(vec) = stack.pop() {
        let center = &vec[0];
        let dist = metric(center.center, target);
        let mid = 1 + (vec.len() - 1) / 2;
        if dist <= eps {
            results.push(center.center);
        }
        if dist < center.radius + eps {
            stack.push(&vec[1..mid]);
        } 
        if dist >= center.radius - eps {
            stack.push(&vec[mid..]);
        }
    }
    results
}

mod tests {

    use crate::vptree::build;
    use crate::vptree::Metric;
    use crate::vptree::Circle;

    #[test]
    fn test_build_full() {
        let dist: Metric<i32> = |n: &i32, m: &i32| {n - m}.abs() as f32;
        let mut _v = vec![-1, 4, -4, 1, -2, -3];
        let vp_vec = build(&mut _v, dist);
        check(&vp_vec.vp_vec, dist);
        println!("Transformed {:?}", vp_vec.vp_vec);
    }

    fn check<T>(vp_vec: &[Circle<T>], dist: Metric<T>) -> () {
        let mut stack: Vec<&[Circle<T>]> = Vec::with_capacity(vp_vec.len());
        stack.push(&vp_vec);
        while let Some(v) = stack.pop() {
            if !v.is_empty() {
                let mid = 1 + (v.len() - 1) / 2;
                for i in 1..mid {
                    assert!(dist(v[0].center, v[i].center) <= v[0].radius);
                }
                for i in mid..v.len() {
                    assert!(dist(v[0].center, v[i].center) >= v[0].radius);
                }
                stack.push(&v[1..mid]);
                stack.push(&v[mid..]);
            }
        }
    }

}
