// So far the implementations we have derived for our NewStruct object will require us to return the value of the fields 
// themselves. When the fields implement Copy this will not be a particularly expensive operation, but when working with
// more complex fields we may not want to Clone the objects. This next implementation will show how we can return references
// to the fields, rather than the fields themselves.

// NOTE: To be comfortable with the implementation you will need to have a basic understanding of the Rust lifetime system.

use std::iter::IntoIterator;
use std::iter::Iterator;

// Since we are returning references to the field values we don't need to constrain the type T.
pub struct NewStruct<T> {
  field1: T,
  field2: T,
  field3: T,
  field4: T,
  field5: T,
}

// The syntax T: 'a is read as T must outlive 'a 
// This means that any reference contained in T must outlive 'a
pub struct NewStructIntoIter<'a,T>
  where T: 'a {
  
  count: usize,
  new_struct: &'a NewStruct<T>,
}

/* Here is the IntoIterator trait definition for reference
 *  pub trait IntoIterator
 *    where Self::IntoIter::Item == Self::Item {
 *
 *    type Item;
 *    type IntoIter: Iterator;
 *
 *    fn into_iter( self ) -> Self::IntoIter;
 *  }
 */
 
 impl<'a,T> IntoIterator for &'a NewStruct<T> 
    where T: 'a {
    
    type Item = &'a T;   // now our iterator will return references to the fields instead of the field values themselves
    type IntoIter = NewStructIntoIter<'a,T>;
    
    fn into_iter( self: Self ) -> NewStructIntoIter<'a,T> {
      
        NewStructIntoIter {
          count: 0 as usize,
          new_struct: self,  // self here is of type &'a NewStruct<T>, so our type now contains a reference to the whole structure
        }
    }
}
