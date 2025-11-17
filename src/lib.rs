#![no_std]

pub use enumorph_derive::Enumorph;

/// Convenience trait around the conversions provided by [`Enumorph`], but with
/// the generics on the functions instead of the trait.
///
/// ```rust
/// use enumorph::{Enumorph, EnumorphAs};
///
/// #[derive(Enumorph)]
/// enum Enum1 {
///     A(A),
///     B(B),
/// }
///
/// #[derive(Enumorph)]
/// enum Enum2 {
///     A(A),
///     C(C),
/// }
///
/// struct A;
/// struct B;
/// struct C;
///
/// let enum_1 = A.widen::<Enum1>();
/// let enum_2 = A.widen::<Enum2>();
/// ```
pub trait EnumorphAs: Sized {
    fn widen<T>(self) -> T
    where
        Self: Into<T>,
    {
        self.into()
    }

    fn narrow<T>(self) -> Result<T, Self>
    where
        Self: TryInto<T, Error = Self>,
    {
        self.try_into()
    }
}

impl<T> EnumorphAs for T {}

pub trait Enumorph<T>: TryFrom<T, Error = T> + Into<T> {}
impl<T, U> Enumorph<U> for T where T: TryFrom<U, Error = U> + Into<U> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumorph_as() {
        #[derive(Debug, PartialEq, Enumorph)]
        enum Enum<T, U> {
            A(A<T>),
            B(B<U>),
        }

        #[derive(Debug, PartialEq)]
        struct A<T>(T);

        #[derive(Debug, PartialEq)]
        struct B<U>(U);

        assert_eq!(A("a").widen::<Enum<&str, u8>>(), Enum::A(A("a")));
        assert_eq!(Enum::<&str, u8>::A(A("a")).narrow::<A<_>>(), Ok(A("a")));

        assert_eq!(B(1).widen::<Enum<&str, u8>>(), Enum::B(B(1)));
        assert_eq!(Enum::<&str, u8>::B(B(1)).narrow::<B<_>>(), Ok(B(1)));
    }

    #[test]
    fn try_from_into() {
        #[derive(Debug, PartialEq, Enumorph)]
        enum A {
            B(B),
            C(C),
            D(D),
        }

        #[derive(Debug, PartialEq)]
        struct B;
        #[derive(Debug, PartialEq)]
        struct C;

        #[derive(Debug, PartialEq, Enumorph)]
        enum D {
            E(E),
        }

        #[derive(Debug, PartialEq)]
        struct E;

        assert!(matches!(A::B(B).try_into(), Ok(B)));
        assert!(matches!(A::C(C).try_into(), Ok(C)));
        assert!(matches!(A::D(D::E(E)).try_into(), Ok(D::E(E))));
        // currently doesn't work
        // assert_eq!(A::D(D::E(E)).try_into(), Ok(E));

        assert!(matches!(B.into(), A::B(B)));
        assert!(matches!(C.into(), A::C(C)));
        assert!(matches!(D::E(E).into(), A::D(D::E(E))));
        // currently doesn't work
        // assert!(matches!(E.into(), A::D(D::E(E))));
    }

    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.pass("tests/ui/pass/*.rs");
        t.compile_fail("tests/ui/compile_fail/*.rs");
    }
}
