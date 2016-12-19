// From the previous example, src4.rs, it may seem that defining an iterator that returns mutable references should be as
// easy as replacing each instance of & with &mut. It turns out that this is not the case. The reason it falls down is 
// that an object can only have one mutable borrow at any given time whereas we can have as many immutable borrows (references)
// as we want. Below, I simulate the replacement and then discuss how the implementation fails to work.

use std::iter::IntoIterator;
use std::iter::Iterator;

pub struct NewStruct<T> {
    field1: T,
    field2: T,
    field3: T,
    field4: T,
    field5: T,
}

pub struct NewStructMutRef<'a,T>
    where T: 'a {
    
    count: usize,
    new_struct: &'a mut NewStruct,  
}

impl<'a,T> IntoIterator for &'a mut NewStruct<T> 
    where T: 'a {
    
    type Item = &'a mut T;
    type IntoIter = NewStructMutRef<'a,T>;
    
    fn into_iter( self: Self ) -> NewStructMutRef<'a,T> {
    
        NewStructMutRef {
            count: 0 as usize,
            new_struct: &'a mut NewStruct,
        }
    }
}

// So far so good, the issue we run into is the lifetime used by the `next` method
// The following has an incorrect implementation of the Iterator trait that mirrors exactly what we did in the reference (&)
// case.
impl<'a,T> Iterator for NewStructMutRef<'a,T> 
    where T: 'a {

    type Item = &'a mut T;
    
    fn next<'b>( self: &'b mut Self ) -> Option<&'a mut T> {
    
        self.count += 1;
        
        match self.count {
            1 => { Some(&mut self.new_struct.field1) },
            2 => { Some(&mut self.new_struct.field2) },
            3 => { Some(&mut self.new_struct.field3) },
            4 => { Some(&mut self.new_struct.field4) },
            5 => { Some(&mut self.new_struct.field5) },
            _ => { None },
        }
    }
}

// The compiler will complain that it cannot infer an appropriate lifetime for the borrow expression due to conflicting requirements
// It suggests changing the lifetime parameter 'b to 'a so that `next` looked like the following:

// fn next<'a>( self: &'a mut Self ) -> Option<&'a mut T>

// However, this will not fix the situation for two reasons:
//  1. That is not how the trait is defined so we will get a method incompatible with trait error
//  2. This new form does not describe the semantics that next is supposed to have.
//       This form is now saying that the lifetime of the NewStructMutRef is the same as the elements it returns which 
//       we know is not the case. In particular, the lifetime 'a contains (or outlives) the NewStructMutRef

// The fact that this will not compile seems odd given that the fields of base structure (NewStruct) are disjoint so we 
// should be able to borrow them individually. The problem is we don't have the semantics to properly tell the compiler that
// the mutable borrow of the field has a shorter lifetime than the mutable borrow of the NewStructMutRef so it craps out.

// You could circumvent this by using unsafe code and *mut T since there is no explicit lifetime restriction when using 
// unsafe pointers.
    
