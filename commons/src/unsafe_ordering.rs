use std::cmp::Ordering;

pub fn unsafe_ordering<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    a.partial_cmp(b).unwrap()
}

#[derive(Debug)]
pub struct UnsafeOrderable<T> {
    pub value: T,
}

impl<T> UnsafeOrderable<T> {
    pub fn new(value: T) -> UnsafeOrderable<T> {
        UnsafeOrderable { value }
    }
}

impl<T> Ord for UnsafeOrderable<T>
where
    T: PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe_ordering(&self.value, &other.value)
    }
}

impl<T> PartialOrd for UnsafeOrderable<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for UnsafeOrderable<T>
where
    T: PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for UnsafeOrderable<T> where T: PartialOrd {}
