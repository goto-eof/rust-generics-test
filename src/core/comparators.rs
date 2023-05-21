pub fn calculate_largest<T>(list: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    if list.is_empty() {
        return None;
    }
    let mut largest = &list[0];
    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }
    return Some(largest.to_owned());
}
