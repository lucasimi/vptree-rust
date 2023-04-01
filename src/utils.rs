fn partition<T: PartialOrd>(vec: &mut [T], k: usize) -> usize {
    if k >= vec.len() {
        return vec.len();
    }
    vec.swap(k, 0);
    let mut higher: usize = 1;
    for j in 1..vec.len() {
        if vec[j] < vec[0] {
            vec.swap(higher, j);
            higher += 1;
        }
    }
    vec.swap(0, higher - 1);
    higher
}

pub fn quick_select<T: PartialOrd>(vec: &mut [T], k: usize) -> () {
    if vec.len() < 2 {
        return;
    }
    let mut _higher: usize = k;
    let mut _vec = &mut vec[..];
    let mut _k = k;
    while _higher != _k + 1 {
        _higher = partition(_vec, _k);
        if k < _higher - 1 {
            _vec = &mut _vec[0.._higher];
        } else {
            _vec = &mut _vec[_higher..];
            _k -= _higher - 1;
        }
    }
}

mod tests {

    use crate::utils::quick_select;

    use super::partition;

    #[test]
    fn test_partition_empty() {
        let mut v: Vec<i32> = vec![];
        partition(&mut v, 1);
    }

    #[test]
    fn test_partition_full() {
        let mut v = vec![-1, 4, -4, 1, -2, -3];
        let h = partition(&mut v, 3);
        for i in 0..h {
            assert_eq!(v[i] <= v[h - 1], true);
        }
        for i in h..v.len() {
            assert_eq!(v[i] > v[h - 1], true);
        }
    }

    #[test]
    fn test_quick_select_empty() {
        let mut v: Vec<i32> = vec![];
        quick_select(&mut v, 2);
    }

    #[test]
    fn test_quick_select_full() {
        let mut _v = vec![-1, 4, -4, 1, -2, -3];
        quick_select(&mut _v, 2);
        assert_eq!(_v[2], -2);
    }

}
