# Enumorph

Derive macro to generate `TryFrom` and `From` implementations for converting between newtype enum variants and their wrapped values.

```rust
#[derive(Enumorph)]
enum Enum<T: ToOwned + ?Sized, U>
where
    U: Display,
{
    A(A<T>),
    B { b: B<U> },
    #[enumorph(ignore)]
		C,
    #[enumorph(ignore)]
		D {
				e: u8,
				f: bool,
		}
}

struct A<T: ToOwned + ?Sized>(T::Owned);

struct B<U: Display>(U);

// Enum::A(A("a"))
Enum::<str, u8>::from(A("a".to_owned()));

// Ok(A("a"))
A::try_from(Enum::<str, u8>::A(A("a".to_owned())));

// Enum::B { b: B(1) }
Enum::<str, u8>::from(B(1));

// Ok(B(1))
B::try_from(Enum::<str, u8>::B(B(1));

// Err(Enum::C)
B::try_from(Enum::<str, u8>::C);
```

## Limitations

If two variants "wrap" the same type, then the resulting From and TryFrom implementations will overlap. In this case, you can wrap the inner type in a newtype:

```rust
#[derive(Enumorph)]
enum T {
    U(U),
		V(V),
}

struct U(String);
struct V(String);
```
