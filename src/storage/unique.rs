use std::any::Any;

pub trait Unique: Any {}

pub(crate) struct UniqueStorage<T: Unique>(pub T);
