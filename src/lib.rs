mod sort;

#[cfg(test)]
mod tests {
    use crate::sort;

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
        let rez = sort::search::binary_search(&v, 100000, 0_usize, v.clone().len()-1_usize);
        assert_eq!(Ok(6400 as usize), rez);
    }

    #[test]
    fn test_search_for_value_binary_search_fail_case() {
        let v = (0..100000).filter(|&i| i % 5 != 0).collect::<Vec<usize>>();
        let rez = sort::search::binary_search(&v, 100001, 0_usize, v.clone().len()-1_usize);
        assert_eq!(Err(0 as usize), rez);
    }
}
