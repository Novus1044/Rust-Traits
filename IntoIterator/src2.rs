// Just like src1.rs this file cannot be compiled as is. We still have to meet the constraint of IntoIter: Iterator which is 
// not satisfied in this source file.
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

// The clause where T: Copy means that NewStructIntoIter only accepts NewStruct objects whose fields are types that implement
// the Copy trait. These would namely be the primitive types like char, bool, i8, u8, i16, u16, etc.
pub struct NewStructIntoIter<T>
	where T: Copy {
		
	count: usize,
	new_struct: NewStruct<T>,
}

/*
 * The IntoIterator trait declaration for your reference.
 *   pub trait IntoIterator where Self::IntoIter::Item == Self::Item {
 *     type Item;
 *     type IntoIter: Iterator;
 *     fn into_iter(self) -> Self::IntoIter;
 *   }
 */

// Now our trait definition requires impl<T> to introduce the generic type parameter into scope so we can use it in our
// implementation
impl<T> IntoIterator for NewStruct<T> 
	where T: Copy {
		
	type Item = T;
	type IntoIter = NewStructIntoIter<T>; // just like before, for this to work NewStructIntoIter needs to implement the Iterator
										  // trait, that is what IntoIter: Iterator means in the declaration above.
										  // So, compilation would fail with an error saying that NewStructIntoIter does not 
										  // implement the Iterator trait.
	fn into_iter( self: Self ) -> NewStructIntoIter<T> {
		
		NewStructIntoIter {
			count: 0 as usize, 
			new_struct: self,             // This is a bitwise copy of self since we are using Copy types, no move occurs here
		}
	}
}



	


