use crate::DataNumberType;

pub trait Merge {

    fn merge(&mut self, other: Self);
}

impl<T: Merge> Merge for Option<T> {
    fn merge(&mut self, mut other: Self) {
        if !self.is_some() {
            *self = other.take();
        } else {
            self.merge(other)
        }
    }
}


pub trait MergeByRef {

    fn merge_by_ref(&mut self, other: &mut Self);
}

impl<T: MergeByRef> MergeByRef for Option<T> {
    fn merge_by_ref(&mut self, other: &mut Self) {
        if !self.is_some() {
            *self = other.take();
        } else {
            self.merge_by_ref(other)
        }
    }
}

// trait AdditionalOperations {
//
//     fn precision_floor<T>(&self, precision: f64) -> T {
//         self.flo
//     }
// }



