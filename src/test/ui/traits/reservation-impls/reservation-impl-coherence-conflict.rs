// compile-fail

// check that reservation impls are accounted for in negative reasoning.

#![feature(rustc_attrs)]

pub trait MyTrait {}
#[rustc_reservation_impl="this impl is reserved"]
impl MyTrait for () {}

pub trait OtherTrait {}
impl OtherTrait for () {}
impl<T: MyTrait> OtherTrait for T {}
//~^ ERROR conflicting implementations

fn main() {}
