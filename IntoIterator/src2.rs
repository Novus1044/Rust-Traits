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

//
