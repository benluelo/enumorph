#[derive(enumorph::Enumorph)]
enum Enum<T> {
    A { b: T, c: u8 },
}

fn main() {}
