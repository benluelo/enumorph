#[derive(Debug, enumorph::Enumorph)]
enum Enum {
    A(u8),
    #[enumorph(ignore)]
    B,
}

fn main() {
    let _: Enum = 0_u8.into();
    let _: u8 = Enum::A(0).try_into().unwrap();
}
