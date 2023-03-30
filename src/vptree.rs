use crate::utils;
use fastrand;

pub trait Metric<T> {

    fn eval(x: T, y: T) -> f32;

}

struct VPNode<'a, T: 'a> {
    data: &'a T,
    dist: f32
}

impl<'a, T: 'a> PartialEq for VPNode<'a, T> {

    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }

}

impl<'a, T: 'a> PartialOrd for VPNode<'a, T> {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }

}

impl<'a, T: 'a> VPNode<'a, T> {

    fn new(x: &'a T) -> VPNode<'a, T> {
        VPNode { data: x, dist: 0.0f32 }
    }

}

fn build_range<T>(_vec: &mut Vec<VPNode<T>>, _range: utils::Range) -> () {
    let mid: usize = _range.mid();
    _vec.swap(0, mid);
}

fn build<T>(vec: &Vec<T>) -> () {
    let mut stack: Vec<utils::Range> = Vec::with_capacity(vec.len());
    let mut vp_vec: Vec<VPNode<T>> = vec.iter()
        .map(|x| VPNode::new(x))
        .collect();
    stack.push(utils::Range::new(0, vec.len()));
    while let Some(rec) = stack.pop() {
        build_range(&mut vp_vec, rec);
        if rec.len() > 1 {
            stack.push(rec.half_left());
            stack.push(rec.half_right());
        }
    }
}

fn build_range_slice<T>(_vec: &mut [VPNode<T>], metric: fn(&T, &T) -> f32) -> () {
    let pivot: usize = fastrand::usize(0.._vec.len());
    let mid: usize = _vec.len() / 2;
    for i in 0.._vec.len() {
        _vec[i].dist = metric(_vec[pivot].data, _vec[i].data);
    }
    utils::quick_select_slice(_vec, mid);
}

fn build_slice<T>(vec: &[T], metric: fn(&T, &T) -> f32) -> () {
    let mut stack: Vec<&mut [VPNode<T>]> = Vec::with_capacity(vec.len());
    let mut vp_vec: Vec<VPNode<T>> = vec.iter()
        .map(|x| VPNode::new(x))
        .collect();
    let mut _vp_vec = &mut vp_vec[..];
    stack.push(_vp_vec);
    while let Some(_vec) = stack.pop() {
        build_range_slice(_vec, metric);
        if _vec.len() > 1 {
            let mid = _vec.len() / 2;
            let (left, right) = _vec.split_at_mut(mid);
            stack.push(left);
            stack.push(right);
        }
    }
}
