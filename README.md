<h1>
  <img src="./doc/logo.png" alt="Logo, just triangles" />
  Code Tour
</h1>

This project is an attempt to improve Rust example-based learning
approach.

Imagine the following example file:

```rust
#[derive(Debug)]
struct S {
    x: i32,
    y: i32,
}

fn main() {
    // Declare something.
    // Because it's an example.
    let a = S { x: 7, y: 42 };

    let b = a.x + a.y;

    // Here is the result!
    let c = b + 1;
}
```

When you run the example with `cargo run --example foo`, you'll see
nothing! It means the author of the example must add `println!` or
`dbg!` instructions everytime. And we are going to miss the comments
too.

Enter `code_tour`.

Let's rewrite the example as such:

```rust
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
```

Let's re-execute the example as previously with `cargo run --example
foo`, and we'll see:

![cargo run example](./doc/cargo_run_example.png)

The example annotations are replicated on the output during the
execution.

An annotation must be a comment of kind `///` or `/** … */` that
introduces a `let` binding. That's all for the moment!
