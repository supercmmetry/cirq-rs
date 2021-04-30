use crate::ops::raw_types::{QId, Gate};

pub trait Decompose {
    fn decompose<T>(&self, qubits: Vec<Box<dyn QId>>) -> T;
}

pub trait HasUnitary {
    fn has_unitary(&self) -> bool;
}

pub trait EqualityValue {
    fn equality_value<T>(&self) -> T;
}

