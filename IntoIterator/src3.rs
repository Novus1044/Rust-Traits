// This implementation of the IntoIterator trait handles an object with completely generic data, provided the generic data
// implements the Clone trait.

use std::iter::IntoIterator;
use std::iter::Iterator;

// As noted above, the only constraint we have enforced is that the type T implements Clone
pub struct NewStruct<T>
  where T: Clone {
  
  field1: T,
  field2: T,
  field3: T,
  field4: T,
  field5: T,
}

pub struct NewStructIntoIter<T>
  where T: Clone {
  
  count: usize,
  new_struct: NewStruct,
}

/* Included is the trait definition for your reference
 *  pub trait IntoIterator
 *    where Self::IntoIter::Item == Self::Item {
 *
 *    type Item;
 *    type IntoIter: Iterator;   // like in the other src files this constraint is not properly handled yet
 *
 *    fn into_iter( self ) -> Self::IntoIter;
 *  }
 */

impl<T> IntoIterator for NewStruct<T> 
  where T: Clone {
  
  type Item = T;
  type IntoIter = NewStructIntoIter;
  
  fn into_iter( self: Self ) -> NewStructIntoIter {
  
    NewStructIntoIter {
      count: 0 as usize,
      new_struct: self,  // ownership of self now belongs to the new_struct field
    }
  }
}
  

