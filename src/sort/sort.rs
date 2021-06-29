fn merge<T: Copy + Ord + std::fmt::Debug>(x1: &[T], x2: &[T], y: &mut [T]) {
    /*
    this will merge the result
    */
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0_usize;
    let mut j = 0_usize;
    let mut k = 0_usize;

    while i < x1.len() && j < x2.len() {
        if x1[i] <= x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }

    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }

    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

pub fn merge_sort<T: Copy + Ord + std::fmt::Debug>(x: &mut [T]) {
    /*
    merge sort algo
    */
    let high = x.len();
    let mid = high / 2;

    if high <= 1 {
        return;
    }
    merge_sort(&mut x[0..mid]);
    merge_sort(&mut x[mid..high]);
    let mut final_result: Vec<T> = x.to_vec();
    merge(&x[0..mid], &x[mid..high], &mut final_result);
    x.copy_from_slice(&final_result);
}


pub fn quick_sort<T: Copy + Ord + std::fmt::Debug>(){
    
}