use crate::utils;
use fastrand;

type Scalar = f32;

type Metric<T> = fn(&T, &T) -> Scalar;

#[derive(Debug)]
struct Circle<'a, T: 'a> {
    center: &'a T,
    radius: Scalar
}

fn get_mid<T>(arr: &[T]) -> usize {
    arr.len() / 2
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

fn process_slice<T>(vec: &mut [Circle<T>], metric: Metric<T>) -> () {
    let pivot: usize = fastrand::usize(0..vec.len());
    vec.swap(pivot, 0);
    for i in 0..vec.len() {
        vec[i].radius = metric(vec[0].center, vec[i].center);
    }
    let tail = &mut vec[1..];
    if !tail.is_empty() {
        let tail_mid: usize = get_mid(tail);
        utils::quick_select(tail, tail_mid);
        vec[0].radius = tail[tail_mid].radius;
    }
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
    if !vp_vec.is_empty() {
        stack.push(vp_vec);
    }
    while let Some(vec) = stack.pop() {
        process_slice(vec, metric);
        let tail = &mut vec[1..];
        let mid_tail = get_mid(tail);
        let (left, right) = tail.split_at_mut(mid_tail);
        if !left.is_empty() {
            stack.push(left);
        }
        if !right.is_empty() {
            stack.push(right);
        }
    }
}

pub fn search<'a, T: 'a>(vpt: &VPTree<'a, T>, target: &T, eps: Scalar) -> Vec<&'a T> {
    let mut results: Vec<&T> = vec![];
    let mut stack: Vec<&[Circle<T>]> = Vec::with_capacity(vpt.vp_vec.len());
    if !vpt.vp_vec.is_empty() {
        stack.push(&vpt.vp_vec);
    }
    while let Some(vec) = stack.pop() {
        let center = &vec[0];
        let dist = (vpt.metric)(center.center, target);
        let tail = &vec[1..];
        let tail_mid = get_mid(tail);
        if dist <= eps {
            results.push(center.center);
        }
        if dist < center.radius + eps {
            let left = &tail[..tail_mid];
            if !left.is_empty() {
                stack.push(left);
            }
        } 
        if dist >= center.radius - eps {
            let right = &tail[tail_mid..];
            if !right.is_empty() {
                stack.push(right);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {

    use crate::vptree::build;
    use crate::vptree::search;
    use crate::vptree::Scalar;
    use crate::vptree::Metric;
    use crate::vptree::Circle;
    use crate::vptree::VPTree;
    use std::collections::HashSet;

    use super::get_mid;

    fn absdist(n: &i32, m: &i32) -> Scalar {
        (*n).saturating_sub(*m).saturating_abs() as f32
    }

    fn search_naive<'a, T: 'a>(vec: &'a [T], dist: Metric<T>, target: &T, eps: Scalar) -> Vec<&'a T> {
        return vec.iter().filter(|&x| dist(target, x) <= eps).collect();
    }

    fn check_vptree<T>(vpt: &VPTree<T>) -> bool {
        let mut stack: Vec<&[Circle<T>]> = Vec::with_capacity(vpt.vp_vec.len());
        if !vpt.vp_vec.is_empty() {
            stack.push(&vpt.vp_vec);
        }
        while let Some(v) = stack.pop() {
            if !v.is_empty() {
                let mid = 1 + get_mid(&v[1..]);
                for i in 1..mid {
                    if (vpt.metric)(v[0].center, v[i].center) > v[0].radius {
                        return false;
                    }
                }
                for i in mid..v.len() {
                    if (vpt.metric)(v[0].center, v[i].center) < v[0].radius {
                        return false;
                    }
                }
                let left = &v[1..mid];
                let right = &v[mid..];
                if !left.is_empty() {
                    stack.push(left);
                }
                if !right.is_empty() {
                    stack.push(right);
                }
            }
        }
        return true;
    }

    #[test]
    fn test_build_empty() {
        let mut vec = vec![];
        let vpt = build(&mut vec, absdist);
        assert!(vpt.vp_vec.is_empty());
        assert!(check_vptree(&vpt));
        assert!(search(&vpt, &0, 1.0).is_empty());
    }

    #[test]
    fn test_build_sample() {
        let mut vec = vec![-1, 4, -4, 1, 2, -3];
        let vpt = build(&mut vec, absdist);
        assert!(check_vptree(&vpt));
        assert_eq!(search(&vpt, &-1, 1.0).len(), 1);
        assert_eq!(search(&vpt, &4, 1.0).len(), 1);
        assert_eq!(search(&vpt, &-4, 1.0).len(), 2);
        assert_eq!(search(&vpt, &1, 1.0).len(), 2);
        assert_eq!(search(&vpt, &2, 1.0).len(), 2);
        assert_eq!(search(&vpt, &-3, 1.0).len(), 2);
        assert_eq!(search(&vpt, &0, 1.0).len(), 2);
    }

    quickcheck! {
        fn prop_build(vec: Vec<i32>) -> bool {
            let vpt = build(&vec, absdist);
            check_vptree(&vpt)
        }

        fn prop_search(vec: Vec<i32>, target: i32, eps: Scalar) -> bool {
            let vpt = build(&vec, absdist);
            let v1 = search_naive(&vec, absdist, &target, eps);
            let v2 = search(&vpt, &target, eps);
            let h1: HashSet<&i32> = HashSet::from_iter(v1);
            let h2: HashSet<&i32> = HashSet::from_iter(v2);
            return h1 == h2;
        }
    }

}
