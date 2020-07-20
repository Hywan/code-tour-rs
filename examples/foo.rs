use code_tour::code_tour;

#[derive(Debug)]
struct S {
    x: i32,
    y: i32,
}

#[code_tour]
fn main() {
    /// Hello
    /// World
    let a = S { x: 7, y: 42 };

    let b = 4 + 3;

    /// Here is the result!
    let c = b + 1;
}
