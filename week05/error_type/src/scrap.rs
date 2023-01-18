use anyhow::{anyhow, Error};
use thiserror::Error;
use std::marker::PhantomData;

#[derive(Error, Debug)]
enum ArrayError {
    #[error("array is full")]
    ArrayFull(anyhow::Error),
    #[error("array is empty")]
    ArrayEmpty(anyhow::Error),
    //#[error("wrong typed pushed: {0}")]
    //WrongPushType(T),
}

#[derive(Debug)]
struct ArrayVec<T: Copy + core::fmt::Debug, const N: usize> {
    values: [Option<PhantomPointer<T>>; N],
    len: usize,
}

#[derive(Debug)]
struct PhantomPointer<T: std::marker::Copy + core::fmt::Debug> {
    ptr: *const T,
    _market: PhantomData<T>
}


impl<T: Copy + core::fmt::Debug, const N: usize> ArrayVec<T, N> {
    fn new() -> Self{
        ArrayVec{
            values: [None; N],
            len: 0,
        }
    }
    fn try_push(&mut self, t: T) -> Result<(), ArrayError> {
        if self.len == N {
            return Err(ArrayError::ArrayFull(anyhow!("There is no space on the array for the value {:?}", t)));
        }
        self.values[self.len] = Some(t);
        self.len += 1;
        Ok(())
    }
    fn len(&self) -> Result<usize, ArrayError> {
        let filtered: Vec<&Option<T>> = self.values.iter().filter(|x| x.is_some()).collect();
        if filtered.len() == 0 {
            return Err(ArrayError::ArrayEmpty(anyhow!("The array is empty")))
        };
        Ok((filtered.len()))
    }
}


