use std::iter::IntoIterator;
use std::iter::Iterator;

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
    new_struct: NewStruct<T>,
}

impl<T> IntoIterator for NewStruct<T>
    where T: Clone {
    
    type Item = T;
    type IntoIter = NewStructIntoIter<T>;
    
    fn into_iter( self: Self ) -> NewStructIntoIter<T> {
    
        NewStructIntoIter {
            count: 0 as usize,
            new_struct: self,  // ownership of self now bleongs to the new_struct field
        }
    }
}

impl<T> Iterator for NewStructIntoIter<T> 
    where T: Clone {
    
    type Item = T;
    
    fn next<'a>( self: &'a mut Self ) -> Option<T> {
    
        self.count += 1;
        
        // Since T may not be Copy, values of type T are moved by default.
        // However, you can't move ownership if you are not the owner which is the case with `self`.
        // `self` is a mutable borrow and therefore we cannot move values out of it. This is why the .clone()
        // method is used on the return values. Clone creates an identical object whose ownership would be separate 
        // from the value that was cloned. If you remove the .clone() method you will get the error 
        // "Moved out of borrowed content", which is exactly what is described above.
        match self.count {
            1 => { Some(self.new_struct.field1.clone()) },
            2 => { Some(self.new_struct.field2.clone()) },
            3 => { Some(self.new_struct.field3.clone()) },
            4 => { Some(self.new_struct.field4.clone()) },
            5 => { Some(self.new_struct.field5.clone()) },
            _ => { None },
        }
    }
}

// Below are two examples:
//  1. This uses a primitive type of i32, so when we call clone we are returning simple bitwise copies of the fields
//  2. Uses heap allocated data which are moved by default. The clone returns a new heap allocated memory whose contents
//     are identical to the field we are currently referring.
fn main() -> () {

    let n_copy = NewStruct {
        field1: 10,
        field2: 11,
        field3: 12,
        field4: 13,
        field5: 14,
    };
    
    let n_move = NewStruct {
        field1: Box::new( 10 ),
        field2: Box::new( 11 ),
        field3: Box::new( 12 ),
        field4: Box::new( 13 ),
        field5: Box::new( 14 ),
    };
    
    for x in n_copy {
        println!( "{}", x );
    }
    
    for x in n_move {
        println!( "{}", *x );  // x is a pointer to heap allocated memory
    }
}

