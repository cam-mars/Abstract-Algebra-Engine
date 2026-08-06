use crate::structures::groups::Group;

pub struct QuotientGroup<T: Group + Clone + PartialEq> {
    parent: Vec<T>, //our group will be a vector(set) or elems of type T(group elems)
    subgroup: Vec<T>,
}


impl<T: Group + Clone + PartialEq> QuotientGroup<T>{
        



}
