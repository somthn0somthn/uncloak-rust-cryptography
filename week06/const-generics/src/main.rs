#[derive(Debug)]
struct GenArrayWrapper<T, const C: usize>{
    inner: [T; C],
}

impl<T: std::marker::Copy + std::default::Default, const C: usize> GenArrayWrapper<T, C> {
    pub fn new() -> Self {
        GenArrayWrapper {
            inner: [Default::default(); C],
        }
    }
    pub fn new_from_slice(slice: &[T; C]) -> Self {
        let slice = slice.to_owned();
        GenArrayWrapper {
            inner: slice,
        }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
}

fn main () {
    let from_new: GenArrayWrapper<i32, 6> = GenArrayWrapper::new();
    println!("test is {:?}", from_new);

    let from_slice: GenArrayWrapper<u32, 7> = GenArrayWrapper::new_from_slice(&[1,2,3,4,5,6,7]);
    println!("from_slice is {:?}", from_slice);

    println!("new.len is {}", from_new.len());

    println!("from_slice index 2 is {:?}", from_slice.get(2));

}