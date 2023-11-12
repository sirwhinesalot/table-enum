# table-enum

<p>
  <a href="https://crates.io/crates/table_enum"><img alt="" src="https://img.shields.io/crates/d/table_enum?label=crates.io" /></a>
</p>

![Icon](https://raw.githubusercontent.com/sirwhinesalot/table-enum/main/table-enum-icon.svg)

A convenient rust macro to create enums with associated constant data.
It lets you associate constant values to enum variants, similar to how [enums work in Java](https://docs.oracle.com/javase/tutorial/java/javaOO/enum.html), 
or how [X macros](https://en.wikipedia.org/wiki/X_macro) are often used in C.

Only the enum tag is ever passed around, the data is accessed through generated const fn functions that match the enum
tag to the relevant data.

This is different from how enums are typically used in Rust, which are actually tagged unions.
(also known as variant types or sum types in computer science theory).

## When would you use this?

An example where non-tagged-union[^1] enums are very useful is compiler or interpreter development. For example:

```rust
use table_enum::table_enum;

table_enum! {
    enum BinaryOp(text: &'static str, precedence: i32, #[default] right_assoc: bool) {
        Add("+", 10, _),
        Sub("-", 10, _),
        Mul("*", 20, _),
        Div("/", 20, _),
        Pow("**", 30, true),
        ...
    }
}
```

[^1]: I really wish Rust and Swift hadn't called their tagged unions "enums". To me enums are meant to be used as in 
this macro. A tagged union should be a kind of *union*.

## How does it work?

The example above expands into the following code:

```rust
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    ...
}
impl BinaryOp {
    const fn text(&self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Pow => "**",
            ...
        }
    }
    const fn precedence(&self) -> i32 {
        match self {
            BinaryOp::Add => 10,
            BinaryOp::Sub => 10,
            BinaryOp::Mul => 20,
            BinaryOp::Div => 20,
            BinaryOp::Pow => 30,
            ...
        }
    }
    // cannot be const because Default::default() is not const
    fn right_assoc(&self) -> bool {
        match self {
            BinaryOp::Add => bool::default(),
            BinaryOp::Sub => bool::default(),
            BinaryOp::Mul => bool::default(),
            BinaryOp::Div => bool::default(),
            BinaryOp::Pow => true,
            ...
        }
    }
}
```

## Changelog

- 0.2.0: Added `#[option]` and`#[default]` attributes to the enum field declarations, see the documentation for more.
- 0.1.0: First version

## Alternative Crates

- [enum-assoc](https://crates.io/crates/enum-assoc): more powerful but less convenient.
- [const-table](https://crates.io/crates/const-table): similar idea but as a derive macro. Honestly a better approach to the same problem.