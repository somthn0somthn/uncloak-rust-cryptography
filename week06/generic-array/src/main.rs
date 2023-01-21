use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::{U5, U6};
use core::slice::SliceIndex;

#[derive(Debug)]
struct GenArrayWrapper<T, N: ArrayLength<T>>{
    array: GenericArray<T, N>
}

impl<T: std::default::Default + Copy, N: ArrayLength<T>> GenArrayWrapper<T, N> {
    fn new() -> Self {
        GenArrayWrapper {
            array: GenericArray::default(),
        }
    }
    fn new_from_slice(slice: &[T]) -> Self {
        let mut gen_array: GenericArray<T, N> = GenericArray::default();
        gen_array.clone_from_slice(slice);

        GenArrayWrapper {
            array: gen_array,
        }
    }
    /* pub fn from_slice(slice: &[T]) -> &GenericArray<T, N> {
        slice.into()
    } */
     


    fn len(&self) -> usize {
        self.array.len()
    
    }
    fn get<I: std::slice::SliceIndex<[T]>>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output> {
        self.array.get(index)
    }
/* 
    pub const fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: ~const SliceIndex<Self>,
    {
        index.get(self)
    } */
}
/* 
pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
where
    I: SliceIndex<[T]>,  */

fn main() {
    let array: GenArrayWrapper<i32, U5> = GenArrayWrapper::new();
    println!("array is {:?}", array);
    println!("length is {}", array.len());
    let value = array.get(1);
    println!("value is {:?}", value);

    let array_from_slice: GenArrayWrapper<i32, U6> = GenArrayWrapper::new_from_slice(&[1,2,3,4,5,6]);
    println!("from_slice is {:?}", array_from_slice);
}



//Using the generic_array, modify the following struct to wrap
//a generically sized array. Implement a new, len, and 
//get_index method for the array. Instantiate your array 
//in a test with the typenum crate. Repeat the exercise 
//with const generics (without generic_array or typenum).