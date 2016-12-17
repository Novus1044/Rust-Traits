use std::iter::IntoIterator;
use std::iter::Iterator;

// Unlike src1.rs, this base structure is generic over any type T
pub struct NewStruct<T> {
  field1: T,
  field2: T,
  field3: T,
  field4: T,
  field5: T,
}

// The clause where: T Copy means that NewStructIntoIter only accepts NewStruct objects whose fields implement the Copy trait.
// It is constraining the types that T can be. Copy types would be primitive types like char, bool, i8, u8, etc.
pub struct NewStructIntoIter<T> 
  where T: Copy {
  
  count: usize,
  new_struct: NewStruct<T>,
}

/* 
 * The IntoIterator trait declaration for your reference.
 *  pub trait IntoIterator
 *    where Self::IntoIter::Item == Self::Item {
 * 
 *    type Item;
 *    type IntoIter: Iterator;
 * 
 *    fn into_iter( self ) -> Self::IntoIter;
 * }
 */
 
// Now our trait implementation requires impl<T> to introduce the generic type parameter T into scope 
impl<T> IntoIterator for NewStruct<T> 
  where T: Copy {
  
  type Item = T;
  type IntoIter = NewStructIntoIter<T>; // for this implementation to be valid, NewStructIntoIter needs to implement Iterator 
                                        // trait. This is precisely what IntoIter: Iterator means in the trait definition above.
  
  fn into_iter( self: Self ) -> NewStructIntoIter<T> {
  
    NewStructIntoIter {
      count: 0 as usize,
      new_struct: self,                 // This is a bitwise copy of self since we are using Copy types, no move occurs here
    }
  }
}
