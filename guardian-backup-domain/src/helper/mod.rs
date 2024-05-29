use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

pub trait COptional {
    type Item;
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct CSome<T>(pub T);
pub struct CNone<T>(PhantomData<T>);

impl<T> COptional for CSome<T> { type Item = T; }
impl<T> COptional for CNone<T> { type Item = T; }


impl<T> Default for CNone<T> {
    fn default() -> Self {
        CNone(PhantomData::default())
    }
}