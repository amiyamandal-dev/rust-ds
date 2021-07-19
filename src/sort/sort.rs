use rand::Rng;

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

fn _partition<T: Copy + Ord + std::fmt::Debug>(x: &mut [T], low: usize, high: usize) -> usize {
    /*
    partition fucntion of quick sort
    */
    let num = high;
    let pivote = x[num];
    let mut i = low as i128 - 1;
    for j in low..(high + 1_usize) {
        if x[j] < pivote {
            i += 1;
            let c = x[(i as usize)];
            x[(i as usize)] = x[j];
            x[j] = c;
        }
    }
    let c = x[(i + 1) as usize];
    x[(i + 1) as usize] = x[num];
    x[num] = c;

    (i + 1) as usize
}

pub fn quick_sort<T: Copy + Ord + std::fmt::Debug>(x: &mut [T], low: usize, high: usize) {
    if low < high {
        let privote_index = _partition(x, low, high);
        quick_sort(x, low, privote_index - 1);
        quick_sort(x, privote_index + 1, high);
    }
}

pub fn heap_sort<T: Copy + Ord + std::fmt::Debug>() {}
