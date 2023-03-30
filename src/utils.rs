#[derive(Copy, Clone)]
pub struct Range {
    start: usize,
    bound: usize
}

impl Range {

    pub fn new(start: usize, bound: usize) -> Range {
        Range {
            start: start,
            bound: bound
        }
    }

    pub fn mid(&self) -> usize {
        self.start + self.bound
    }

    pub fn len(&self) -> usize {
        self.bound - self.start
    }

    pub fn slice_left(&self, split: usize) -> Range {
        Range::new(self.start, split)
    }

    pub fn slice_right(&self, split: usize) -> Range {
        Range::new(split, self.bound)
    }

    pub fn half_left(&self) -> Range {
        self.slice_left((self.start + self.bound) / 2)
    }

    pub fn half_right(&self) -> Range {
        self.slice_right((self.start + self.bound) / 2)
    }
        
}

fn _partition_slice<T: PartialOrd>(vec: &mut [T], k: usize) -> usize {
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

fn _partition<T: PartialOrd>(vec: &mut Vec<T>, range: Range, k: usize) -> usize {
    vec.swap(k, range.start);
    let mut higher: usize = range.start + 1;
    for j in range.start..range.bound {
        if vec[j] < vec[range.start] {
            vec.swap(higher, j);
            higher += 1;
        }
    }
    vec.swap(range.start, higher - 1);
    higher
}

fn _quick_select_slice<T: PartialOrd>(vec: &mut [T], k: usize) -> () {
    let mut _higher: usize = k;
    let mut _vec = &mut vec[..];
    let mut _k = k;
    while _higher != _k + 1 {
        _higher = _partition_slice(_vec, _k);
        if k < _higher {
            _vec = &mut _vec[0.._higher];
        } else {
            _vec = &mut _vec[_higher..];
            _k -= _higher - 1;
        }
    }
}

fn _quick_select<T: PartialOrd>(vec: &mut Vec<T>, range: Range, k: usize) -> () {
    let mut _range: Range = range;
    let mut _higher: usize = k;
    while _higher != k + 1 {
        _higher = _partition(vec, _range, k);
        if k < _higher {
            _range = _range.slice_left(_higher)
        } else {
            _range = _range.slice_right(_higher)
        }
    }
}

pub fn quick_select<T: PartialOrd>(vec: &mut Vec<T>, k: usize) -> () {
    _quick_select(vec, Range::new(0, vec.len()), k);
}

pub fn quick_select_slice<T: PartialOrd>(vec: &mut [T], k: usize) -> () {
    _quick_select_slice(vec, k);
}

mod tests {

    use crate::utils::quick_select;
    use crate::utils::quick_select_slice;

    #[test]
    fn test_quick_select_full() {
        let mut _v = vec![-1, 4, -4, 1, -2, -3];
        quick_select(&mut _v, 2);
        assert_eq!(_v[2], -2);
    }

    #[test]
    fn test_quick_select_slice_full() {
        let mut _v = vec![-1, 4, -4, 1, -2, -3];
        quick_select_slice(&mut _v, 2);
        assert_eq!(_v[2], -2);
    }
}
