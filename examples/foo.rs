use code_tour::code_tour;

#[derive(Debug)]
struct S {
    x: i32,
    y: i32,
}

#[code_tour]
fn main() {
    /// Declare something.
    /// Because it's an example.
    let a = S { x: 7, y: 42 };

    let b = a.x + a.y;

    /// Here is the result!
    let c = b + 1;
}
