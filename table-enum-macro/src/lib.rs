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
/// ```ignore
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
/// 
/// There are two convenience attributes that can be added for each field:
/// 
/// ```ignore
/// use table_enum::table_enum;
///
/// table_enum! {
///     enum BinaryOp(text: &'static str, #[option] precedence: i32, #[default] right_assoc: bool) {
///         Add("+", _, _),
///         Sub("-", _, _),
///         Mul("*", 20, _),
///         Div("/", 20, _),
///         Pow("**", 30, true),
///         ...
///     }
/// }
/// ```
/// 
/// The `#[option]` convenience attribute lets you write the values directly but wraps them implicitly into `Some(value)`.
/// If instead of a value you write `_`, it becomes `None`. 
/// The return type when you invoke that field's getter is also changed to `Option<FieldType>`.
/// 
/// The #[default] field works the same way but instead of an `Option` type, `_` maps to `Default::default()`.
/// Because `Default::default()` is not `const fn`, neither is the generated function.
pub fn table_enum(input: TokenStream) -> TokenStream {
    table_enum_core(input.into()).into()
}
