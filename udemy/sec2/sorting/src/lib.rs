//Bubble sort V1 Time O(n^2)
//parameter : a mutable slice of type T
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    for _ in 0..v.len() {
        for i in 0..v.len() - 1 {
            if v[i + 1] < v[i] {
                v.swap(i, i + 1)
            }
        }
    }
}

//Bubble sort V2 optimised same complexity.
//Returns directly if the array is already sorted
//And we avoid
pub fn bubble_sort_v2<T: PartialOrd>(v: &mut [T]) {
    for pass in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - pass {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![1, 13, 4, 2, 7, 6, 88, 14];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 6, 7, 13, 14, 88]);
    }

    #[test]
    fn test_bubble_sort_v2() {
        let mut v2 = vec![1, 13, 4, 2, 7, 6, 88, 14];
        bubble_sort_v2(&mut v2);
        assert_eq!(v2, vec![1, 2, 4, 6, 7, 13, 14, 88]);
    }
}
