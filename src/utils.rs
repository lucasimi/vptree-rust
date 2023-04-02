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
    if k >= vec.len() {
        return;
    }
    let mut higher: usize = k;
    let mut arr = &mut vec[..];
    let mut idx = k;
    while higher != idx + 1 {
        higher = partition(arr, idx);
        if higher > idx + 1 {
            arr = &mut arr[..higher];
        } else if higher < idx + 1 {
            arr = &mut arr[higher..];
            idx -= higher;
        }
    }
}

mod tests {

    use crate::utils::quick_select;

    use super::partition;

    #[test]
    fn test_partition_empty() {
        let mut v: Vec<i32> = vec![];
        assert_eq!(partition(&mut v, 0), 0);
        assert_eq!(partition(&mut v, 1), 0);
        assert_eq!(partition(&mut v, 2), 0);
    }

    #[test]
    fn test_partition_singleton() {
        let mut v: Vec<i32> = vec![0];
        assert_eq!(partition(&mut v, 0), 1);
        assert_eq!(partition(&mut v, 1), 1);
        assert_eq!(partition(&mut v, 2), 1);
    }

    #[test]
    fn test_partition_full() {
        let mut v = vec![-1, 4, -4, 1, -2, -3];
        let k = 3;
        let val = v[k];
        let h = partition(&mut v, k);
        assert_eq!(v[h - 1], val);
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
        quick_select(&mut v, 0);
        quick_select(&mut v, 1);
        quick_select(&mut v, 2);
    }

    #[test]
    fn test_quick_select_singleton() {
        let mut v: Vec<i32> = vec![0];
        quick_select(&mut v, 0);
        quick_select(&mut v, 1);
        quick_select(&mut v, 2);
    }

    #[test]
    fn test_quick_select_full() {
        let mut v = vec![-1, 4, -4, 1, -2, -3];
        let k = 2;
        quick_select(&mut v, k);
        for i in 0..k {
            assert_eq!(v[i] <= v[k], true);
        }
        for i in k..v.len() {
            assert_eq!(v[i] >= v[k], true);
        }
    }

}
