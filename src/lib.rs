mod find_first_and_last_position_of_element_in_sorted_array;
mod linked_list;
mod search_in_rotated_sorted_array;
mod sort;
mod swap_nodes_in_pairs;

#[cfg(test)]
mod tests {
    use crate::sort::{
        self,
        sort::{merge_sort, quick_sort},
    };
    use rand::{prelude::SliceRandom, thread_rng};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_search_for_value_liner_serach() {
        let v = (0..100000).filter(|&i| i % 5 != 0).collect::<Vec<usize>>();
        let rez = sort::search::linear_search(&v, 8001);
        assert_eq!(Ok(6400 as usize), rez);
    }

    #[test]
    fn test_search_for_value_binary_search() {
        let v = (0..100000).filter(|&i| i % 5 != 0).collect::<Vec<usize>>();
        let rez = sort::search::binary_search(&v, 999, 0_usize, v.clone().len() - 1_usize);
        assert_eq!(Ok(799 as usize), rez);
    }

    #[test]
    fn test_search_for_value_binary_search_fail_case() {
        let v = (0..100000).filter(|&i| i % 5 != 0).collect::<Vec<usize>>();
        let rez = sort::search::binary_search(&v, 100001, 0_usize, v.clone().len() - 1_usize);
        assert_eq!(Err(79999 as usize), rez);
    }

    #[test]
    fn test_merge_sort() {
        let sorted_vec = (0..100000000)
            .filter(|&i| i % 5 != 0)
            .collect::<Vec<usize>>();
        let mut unsoted_vec = sorted_vec.clone();
        unsoted_vec.shuffle(&mut thread_rng());
        merge_sort(&mut unsoted_vec);
        assert_eq!(sorted_vec, unsoted_vec);
    }

    #[test]
    fn test_quick_sort() {
        let sorted_vec = (0..100).filter(|&i| i % 5 != 0).collect::<Vec<usize>>();
        let mut unsoted_vec = sorted_vec.clone();
        let high = unsoted_vec.len() - 1;
        quick_sort(&mut unsoted_vec, 0, high);
        assert_eq!(sorted_vec, unsoted_vec);
    }
}
