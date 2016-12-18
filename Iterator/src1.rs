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
   *
   
   
