use anyhow::{anyhow, Error};
use core::ops::Deref;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Error, Debug)]
enum ArrayError {
    #[error("array is full")]
    ArrayFull(anyhow::Error),
    #[error("array is empty")]
    ArrayEmpty(anyhow::Error),
}
#[derive(Debug)]
struct ArrayVec<T: Copy + core::fmt::Debug + PartialEq, const N: usize> {
    values: [Option<PhantomPointer<T>>; N],
    len: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PhantomPointer<T: std::marker::Copy + core::fmt::Debug + PartialEq> {
    ptr: *const T,
    _marker: PhantomData<T>,
}

impl<T: Copy + core::fmt::Debug + PartialEq> Deref for PhantomPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: Copy + core::fmt::Debug + PartialEq, const N: usize> ArrayVec<T, N> {
    fn new() -> Self {
        ArrayVec {
            values: [None; N],
            len: 0,
        }
    }
    fn try_push(&mut self, t: T) -> Result<(), ArrayError> {
        if self.len == N {
            return Err(ArrayError::ArrayFull(anyhow!(
                "There is no space on the array for the value {:?}",
                t
            )));
        }
        let phantom_pointer = PhantomPointer {
            ptr: &t,
            _marker: PhantomData,
        };
        self.values[self.len] = Some(phantom_pointer);
        self.len += 1;
        Ok(())
    }
    //so for the the malicious featuer pop and derefernce a PhantomPointer that contains a null pointer,
    //See the end of the ChatGPT discussion to continue
    fn len(&self) -> Result<usize, ArrayError> {
        let mut counter = 0;
        for i in self.values {
            if i.is_some() {
                counter += 1;
            }
        }
        if counter == 0 {
            return Err(ArrayError::ArrayEmpty(anyhow!("The array is empty")));
        };
        Ok(counter)
    }
    fn try_pop(&mut self) -> Result<T, ArrayError> {
        if self.len == 0 {
            return Err(ArrayError::ArrayEmpty(anyhow!("The array is empty")));
        }
        let result = self.values[self.len - 1];
        self.values[self.len - 1] = None;
        self.len -= 1;
        Ok(*result.unwrap())
    }
    fn try_push_malicious(&mut self, t: T) -> Result<(), ArrayError> {
        if self.len == N {
            return Err(ArrayError::ArrayFull(anyhow!(
                "There is no space on the array for the value {:?}",
                t
            )));
        }
        let null_phantom_pointer = PhantomPointer {
            ptr: std::ptr::null(),
            _marker: PhantomData,
        };
        self.values[self.len] = Some(null_phantom_pointer);
        self.len += 1;
        Ok(())
    }
    fn has_malicious(&self) -> bool {
        let null_phantom_pointer = PhantomPointer {
            ptr: std::ptr::null(),
            _marker: PhantomData,
        };
        if self.values.contains(&Some(null_phantom_pointer)) {
            return true;
        }
        false
    }
}

/* #[test]
fn test_has_malicious() {
    let mut test_array: ArrayVec<_, 3> = ArrayVec::new();
    let a = PhantomPointer {
        ptr: &true,
        _marker: PhantomData,
    };
    let b = PhantomPointer {
        ptr: &false,
        _marker: PhantomData,
    };
    let c = PhantomPointer {
        ptr: &true,
        _marker: PhantomData,
    };

    test_array.try_push(a);

    test_array.try_push(b);

    //array.try_push(c)
    test_array.try_push_malicious(c);

    assert!(!test_array.has_malicious())
} */

fn main() {
    let mut array: ArrayVec<_, 3> = ArrayVec::new();
    println!("array length is {:?}", array.len());

    let a = PhantomPointer {
        ptr: &true,
        _marker: PhantomData,
    };
    let b = PhantomPointer {
        ptr: &false,
        _marker: PhantomData,
    };
    let c = PhantomPointer {
        ptr: &100,
        _marker: PhantomData,
    };
    let d = PhantomPointer {
        ptr: &false,
        _marker: PhantomData,
    };
    let e = PhantomPointer {
        ptr: &false,
        _marker: PhantomData,
    };

    let first_result = array.try_push(a);

    array.try_push(b);

    //will lead to a seg_fault if popped
    let final_result = array.try_push_malicious(d);
    //let final_result = array.try_push(d);

    println!("array: \n{:?}", array);
    println!(
        "first_result {:?}, final_result {:?}",
        first_result, final_result
    );

    let has_malicious = array.has_malicious();
    println!("has_malicious = {has_malicious}");

    let popped = array.try_pop();

    println!("popper is {:?}", popped.unwrap());
    println!("array: \n{:?}", array);
}
