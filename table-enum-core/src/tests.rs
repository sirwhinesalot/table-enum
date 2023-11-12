#![cfg(test)]

use crate::table_enum_core;
use quote::quote;

#[test]
fn planets() {
    let before = quote! {
        pub enum Planets(mass: f64, radius: f64) {
            Mercury (3.303e+23, 2.4397e6),
            Venus   (4.869e+24, 6.0518e6),
            Earth   (5.976e+24, 6.37814e6),
            Mars    (6.421e+23, 3.3972e6),
            Jupiter (1.9e+27,   7.1492e7),
            Saturn  (5.688e+26, 6.0268e7),
            Uranus  (8.686e+25, 2.5559e7),
            Neptune (1.024e+26, 2.4746e7),
        }
    };
    let after = table_enum_core(before);
    let expected = quote! {
        pub enum Planets {
            Mercury,
            Venus,
            Earth,
            Mars,
            Jupiter,
            Saturn,
            Uranus,
            Neptune,
        }
        impl Planets {
            pub const fn mass(&self) -> f64 {
                match self {
                    Planets::Mercury => 3.303e+23,
                    Planets::Venus => 4.869e+24,
                    Planets::Earth => 5.976e+24,
                    Planets::Mars => 6.421e+23,
                    Planets::Jupiter => 1.9e+27,
                    Planets::Saturn => 5.688e+26,
                    Planets::Uranus => 8.686e+25,
                    Planets::Neptune => 1.024e+26,
                }
            }
            pub const fn radius(&self) -> f64 {
                match self {
                    Planets::Mercury => 2.4397e6,
                    Planets::Venus => 6.0518e6,
                    Planets::Earth => 6.37814e6,
                    Planets::Mars => 3.3972e6,
                    Planets::Jupiter => 7.1492e7,
                    Planets::Saturn => 6.0268e7,
                    Planets::Uranus => 2.5559e7,
                    Planets::Neptune => 2.4746e7,
                }
            }
        }
    };
    assert_eq!(after.to_string(), expected.to_string());
}

#[test]
fn option() {
    let before = quote! {
        pub enum BinaryOp(text: &'static str, #[option] precedence: i32, #[default] right_assoc: bool) {
            Add("+", _, _),
            Sub("-", _, _),
            Mul("*", 20, _),
            Div("/", 20, _),
            Pow("**", 30, true),
        }
    };
    let after = table_enum_core(before);
    let expected = quote! {
        pub enum BinaryOp {
            Add,
            Sub,
            Mul,
            Div,
            Pow,
        }
        impl BinaryOp {
            pub const fn text(&self) -> &'static str {
                match self {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Pow => "**",
                }
            }
            pub const fn precedence(&self) -> Option<i32> {
                match self {
                    BinaryOp::Add => None,
                    BinaryOp::Sub => None,
                    BinaryOp::Mul => Some(20),
                    BinaryOp::Div => Some(20),
                    BinaryOp::Pow => Some(30),
                }
            }
            pub fn right_assoc(&self) -> bool {
                match self {
                    BinaryOp::Add => bool::default(),
                    BinaryOp::Sub => bool::default(),
                    BinaryOp::Mul => bool::default(),
                    BinaryOp::Div => bool::default(),
                    BinaryOp::Pow => true,
                }
            }
        }
    };
    assert_eq!(after.to_string(), expected.to_string());
}