// NOTE: This example cannot be compiled...yet. The IntoIterator trait constrains one of its associated types to only those
//       that implement the Iterator trait. We will continue this example in the Iterator directory of this repository to
//       demonstrate a full working example.

// The standard crate, std, is always imported by default so the following use statements are unnecessary. 
// I included them to show where the traits are declared.
use std::iter::IntoIterator;    
use std::iter::Iterator;       

// The NewStruct type is the object we would ultimately like to iterate over. In this case iterating could be as simple as
// walking through all the fields and printing their values.

// NOTE: This structure could be made generic over any type T but I used a primitive type for the fields for simplicity
#[derive(Copy,Clone)]
pub struct NewStruct {
  field1: i32,
  field2: i32,
  field3: i32,
  field4: i32,
  field5: i32,
}

// Introduce a newtype that wraps the NewStruct and provides additional information that will allow us to easily iterate through
// the NewStruct. We can use the `count` field to know when to stop iterating. For example, if we wanted
// to walk through the structure and print the field's values as we go we can continually update the `count` each time until it 
// is equal to five. Once that is the case the iteration will end and we know we have gone completely through our object.
struct NewStructIntoIter{
  count: usize,
  new_struct: NewStruct,
}

/* Included is the declaration of the IntoIterator trait for reference 
 * pub trait IntoIterator 
 *  where Self::IntoIter::Item == Self::Item {
 *  
 *  type Item;
 *  type IntoIter: Iterator;   // this constraint will NOT be met in this example and therefore cannot be executed as is 
 *                             // I will show you how to do this in the Iterator Trait repository
 *   
 *  fn into_iter(self) -> Self::IntoIter;
 * }
 */

// Now it is just a matter of implementing the IntoIterator trait for the NewStruct. The method for the IntoIterator trait
// is into_iter() which will simply take our NewStruct and return a NewStructIntoIter
impl IntoIterator for NewStruct {

  type Item = i32;                    // the `Items` are what the Iterator should return as it walks through its elements
  type IntoIter = NewStructIntoIter;  // this implementation is INCOMPLETE, see the note in the trait declaration above
 
  // into_iter() takes a NewStruct and returns a NewStructIntoIter which has a field `count` that will allow us to track
  // where we are in the iteratation process.
  fn into_iter( self: NewStruct ) -> NewStructIntoIter {
  
      NewStructIntoIter {
        count: 0 as usize,
        new_struct: self,  // self, a NewStruct, is Copy so we don't have to worry about ownership/moves
      }
   }
}
