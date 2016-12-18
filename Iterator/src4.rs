use std::iter::IntoIterator;
use std::iter::Iterator;

pub struct NewStruct<T> {
    field1: T,
    field2: T,
    field3: T,
    field4: T,
    field5: T,
}

pub struct NewStructIterRef<'a,T> 
    where T: 'a {
    
    count: usize,
    new_struct: &'a NewStruct<T>,
}

impl<'a,T> IntoIterator for &'a NewStruct<T> 
    where T: 'a {

    type Item = &'a T;
    type IntoIter = NewStructIterRef<'a,T>;
    
    fn into_iter( self: Self ) -> NewStructIterRef<'a,T> {
    
        NewStructIterRef {
            count: 0 as usize,
            new_struct: self,  // self is of type &'a NewStruct<T> so our type now contains a reference to the whole structure
        }
    }
}

impl<'a,T> Iterator for NewStructIterRef<'a,T> {
    
    type Item = &'a T;
    
    fn next<'b>( self: &'b mut Self ) -> Option<&'a T> {
    
        self.count += 1;
        
        match self.count {
            1 => { Some(&self.new_struct.field1) },
            2 => { Some(&self.new_struct.field2) },
            3 => { Some(&self.new_struct.field3) },
            4 => { Some(&self.new_struct.field4) },
            5 => { Some(&self.new_struct.field5) },
            _ => { None },
        }
    }
}

fn main() -> () {

    let n_copy = NewStruct {
        field1: 10,
        field2: 11,
        field3: 12,
        field4: 13,
        field5: 14,
    };
    
    let n_clone = NewStruct {
        field1: Box::new( 10 ),
        field2: Box::new( 11 ),
        field3: Box::new( 12 ),
        field4: Box::new( 13 ),
        field5: Box::new( 14 ),
    };
    
    // Each for loop will return an immutable reference to the structures fields, no copying or cloning 
    for x in &n_copy {
        println!( "{}", *x );
    }
    
    for x in &n_clone {
        println!( "{}", *x );
    }
}
    
