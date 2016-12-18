use std::iter::IntoIterator;
use std::iter::Iterator;

#[derive(Copy,Clone)]
pub struct NewStruct<T> 
	where T: Copy {
   
   	field1: T,
	field2: T,
	field3: T,
	field4: T,
	field5: T,
}

pub struct NewStructIntoIter<T> 
	where T: Copy {
  
  	count: usize,
	new_struct: NewStruct<T>,
}
  
impl<T> IntoIterator for NewStruct<T> 
	where T: Copy {
	
	type Item = T;
	type IntoIter = NewStructIntoIter<T>; 
  
  	fn into_iter( self: Self ) -> NewStructIntoIter<T> {
  
  		NewStructIntoIter {
			count: 0 as usize,
			new_struct: self,                
		}
	}
}

// The above was from the previous src1.rs file excluding comments
  
// Here is the Iterator trait definition for reference
/*  pub trait Iterator {
 *
 *    type Item;
 *
 *    fn next<'a>( self: &'a mut Self ) -> Option<Self::Item>;
 *
 *    ... (loads of other functions, the other functions are called iterator adapters)
 *  }
 */
 
 impl<T> Iterator for NewStructIntoIter<T> 
 	where T: Copy {
 
 	type Item = T;
	
	fn next<'a>( self: &'a mut Self ) -> Option<T> {
		
		self.count += 1; 
		
		// Since T is constrained to Copy types the values returned will be copies of the fields, no moves occur.
		match self.count {
			1 => { Some(self.new_struct.field1) },
			2 => { Some(self.new_struct.field2) },
			3 => { Some(self.new_struct.field3) },
			4 => { Some(self.new_struct.field4) },
			5 => { Some(self.new_struct.field5) },
			_ => { None },
		}
	}
}

fn main() -> () {

	let n = NewStruct {
		field1: 10 as u64,  // could define all these fields "as u64" but you only need to do it once since NewStruct is only
		field2: 11,         // generic over a single type parameter T. Once we define T, then for this to compile the other fields
		field3: 12,         // must also be of type T.
		field4: 13,
		field5: 14,
	};
	
	for x in n {
		println!( "{}", x );
	}
}
	
	
