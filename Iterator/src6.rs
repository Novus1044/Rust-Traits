// Here is an implementation to return mutable pointers to the structure's fields since we saw that was not possible to do 
// using mutable references.

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
    new_struct: &'a mut NewStruct<T>,
}

impl<'a,T> IntoIterator for &'a mut NewStruct<T> {

    type Item = *mut T;
    type IntoIter = NewStructMutRef<'a,T>;

    fn into_iter( self: Self ) -> NewStructMutRef<'a,T> {
        
        NewStructMutRef {
            count: 0 as usize,
            new_struct: self,
        }
    }
}

impl<'a,T> Iterator for NewStructMutRef<'a,T> {

    type Item = *mut T;
    
    fn next<'b>( self: &'b mut Self ) -> Option<*mut T> {
    
        self.count += 1;
        
        match self.count {
            1 => { Some(&mut self.new_struct.field1 as *mut T) },
            2 => { Some(&mut self.new_struct.field2 as *mut T) },
            3 => { Some(&mut self.new_struct.field3 as *mut T) },
            4 => { Some(&mut self.new_struct.field4 as *mut T) },
            5 => { Some(&mut self.new_struct.field5 as *mut T) },
            _ => { None },
        }
    }
}

fn main() -> () {

    let mut n_copy = NewStruct {
        field1: 1 as i32,
        field2: 2,
        field3: 3,
        field4: 4,
        field5: 5,
    };
    
    let mut n_clone = NewStruct {
        field1: Box::new( 1 as i32 ),
        field2: Box::new( 2 ),
        field3: Box::new( 3 ),
        field4: Box::new( 4 ),
        field5: Box::new( 5 ),
    };
    
    for x in &mut n_copy {
        unsafe {
            *x = 10;
        }
    }
    
    for x in &mut n_clone {
        unsafe {
            *x = Box::new( 10 );
        }
    }
    
    println!( "{}", n_copy.field1 );   // output: 10
    println!( "{}", n_copy.field5 );   // output: 10
    
    println!( "{}", *n_clone.field1 ); // output: 10
    println!( "{}", *n_clone.field5 ); // output: 10
 }
    
