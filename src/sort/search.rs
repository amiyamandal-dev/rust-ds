pub fn linear_search<T: Ord + std::fmt::Debug>(v: &[T], i: T) -> Result<usize, usize> {
    for (p, x) in v.iter().enumerate() {
        if *x == i {
            return Ok(p);
        } else if *x > i {
            return Err(p);
        }
    }
    Err(0)
}

pub fn binary_search<T: Ord + std::fmt::Debug>(
    v: &[T],
    i: T,
    start_index: usize,
    end_index: usize,
) -> Result<usize, usize> {
    let mid = (start_index + end_index) / 2 as usize;
    let mid_element = &v[mid];
    if *mid_element == i {
        return Ok(mid);
    } else if *mid_element > i {
        return binary_search(v, i, start_index, mid - 1_usize);
    } else if *mid_element < i {
        if start_index <= mid {
            return Err(mid);
        } else {
            return binary_search(v, i, mid + 1_usize, end_index);
        }
    } else {
        return Err(mid);
    }
}
