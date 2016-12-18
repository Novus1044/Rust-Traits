use std::iter::IntoIterator;
use std::iter::Iterator;

#[derive(Copy,Clone)]
pub struct NewStruct {
    field1: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32,
}

pub struct NewStructIntoIter {
    count: usize,
    new_struct: NewStruct,
}

impl IntoIterator for NewStruct {

    type Item = i32;
    type IntoIter = NewStructIntoIter;
  
    fn into_iter( self: NewStruct ) -> NewStructIntoIter {
    
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
 
  // The next method takes a mutable reference -- so it can update our `tracking` variable such as a counter
  // or length which we use to track the progress of the iteration and returns an Optional value, either an
  // item from our object or None. Once `next` returns None the iteration is done. Please read the explanation.txt
  // doc in the foler to see the expansion of the for loop syntax to see why.
    
impl Iterator for NewStructIntoIter {
      
    type Item = i32;
      
    fn next<'a>( self: &'a mut Self ) -> Option<i32> {   // removed syntatic sugar of &'a mut self and added the lifetime param
      
        // first we are going to bump the count by 1 since IntoIterator initially set it to zero
        // also, note this is why next takes a mutable reference, so we can do these types of update operations
        self.count += 1;  
        
        // now match on the count to see where we are in the iteration process. You can either do cascading if/else or match
        match self.count {
            1 => { Some(self.new_struct.field1) },   // if the count is one return the first field of the base structure
            2 => { Some(self.new_struct.field2) },   // there is nothing dictating how this should be done, it is user defined
            3 => { Some(self.new_struct.field3) },
            4 => { Some(self.new_struct.field4) },
            5 => { Some(self.new_struct.field5) },
            _ => { None },
        }
        // once our count exceeds 5 the for loop will halt operations
    }
}

// With IntoIterator defined for NewStruct and Iterator defined for NewStructIntoIterator we can now use the for loop syntax
fn main() -> () {
    
    let n: NewStruct = NewStruct {
        field1: 1, 
        field2: 2,
        field3: 3,
        field4: 4,
        field5: 5,
    };
    
    for x in n {
        println!( "{}", x );
    }
    
    // NOTE: you can also write the first line of the for loop as "for x in n.into_iter()" as well
}

// This code prints the field values of the NewStruct object n
// 1
// 2
// 3
// 4
// 5

          
   
   
