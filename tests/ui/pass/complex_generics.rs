use std::borrow::ToOwned;
use std::fmt::{Debug, Display};

#[derive(Debug, enumorph::Enumorph)]
enum Enum<T: ToOwned + ?Sized, U>
where
    U: Display,
    T::Owned: Debug,
{
    A(A<T>),
    B(B<U>),
}

#[derive(Debug)]
struct A<T: ToOwned + ?Sized>(T::Owned);

#[derive(Debug)]
struct B<U: Display>(U);

fn main() {
    let _: Enum<str, u8> = A("a".to_owned()).into();
    let _: A<str> = Enum::<str, u8>::A(A("a".to_owned())).try_into().unwrap();
    let _: Enum<str, u8> = B(1).into();
    let _: B<u8> = Enum::<str, u8>::B(B(1)).try_into().unwrap();
}
