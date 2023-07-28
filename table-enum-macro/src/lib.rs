#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use table_enum_core::table_enum_core;

#[proc_macro_error]
#[proc_macro]
/// Creates a table-like enum.
///
/// # Example:
///
/// ```rust
/// use table_enum::table_enum;
///
/// table_enum! {
///     enum BinaryOp(text: &'static str, precedence: i32, right_assoc: bool) {
///         Add("+", 10, false),
///         Sub("-", 10, false),
///         Mul("*", 20, false),
///         Div("/", 20, false),
///         Pow("**", 30, true),
///         ...
///     }
/// }
///
/// fn example() {
///   println!("{}", BinaryOp.Add.text());
/// }
/// ```
pub fn table_enum(input: TokenStream) -> TokenStream {
    table_enum_core(input.into()).into()
}
