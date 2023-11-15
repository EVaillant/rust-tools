use rust_tools::merge_sort;

#[test]
fn test_merge_sort_01() {
    let mut data = vec![];
    merge_sort(&mut data);
    assert_eq!(data.len(), 0);
}

#[test]
fn test_merge_sort_02() {
    let mut data = vec![10, 5];
    merge_sort(&mut data);
    assert_eq!(data.len(), 2);
    assert_eq!(data, vec![5, 10]);
}

#[test]
fn test_merge_sort_03() {
    let mut data = vec![10, 5, 8, 1];
    merge_sort(&mut data);
    assert_eq!(data.len(), 4);
    assert_eq!(data, vec![1, 5, 8, 10]);
}

#[test]
fn test_merge_sort_04() {
    let mut data = vec![10, 5, 8, 2, 3, 9, 1, 12];
    merge_sort(&mut data);
    assert_eq!(data.len(), 8);
    assert_eq!(data, vec![1, 2, 3, 5, 8, 9, 10, 12]);
}
