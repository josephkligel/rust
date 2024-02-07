mod front_of_house; 

fn deliver_order() {}

mod back_of_house;

pub use crate::front_of_house::hosting; 
// This is the idiomatic version that explains where future
// methods come from

mod customer;

