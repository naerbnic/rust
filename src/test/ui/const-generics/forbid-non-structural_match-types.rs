#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

#[derive(PartialEq, Eq)]
struct A;

struct B<const X: A>; // ok

struct C;

struct D<const X: C>; //~ ERROR `C` must be annotated with `#[derive(PartialEq, Eq)]`

fn main() {}
