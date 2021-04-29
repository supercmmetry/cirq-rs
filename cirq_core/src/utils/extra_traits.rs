use dyn_clonable::dyn_clone::DynClone;
use dyn_clonable::dyn_clone;

pub trait Hashable: DynClone {
    fn hash(&self) -> u64;
}

dyn_clone::clone_trait_object!(Hashable);