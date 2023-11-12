use table_enum::table_enum;

table_enum! {
    #[derive(Debug)]
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
}

table_enum! {
    pub enum BinaryOp(text: &'static str, #[option] precedence: i32, #[default] right_assoc: bool) {
        Add("+", _, _),
        Sub("-", _, _),
        Mul("*", 20, _),
        Div("/", 20, _),
        Pow("**", 30, true),
    }
}

#[test]
fn venus_mass() {
    assert_eq!(Planets::Venus.mass(), 4.869e+24);
}

#[test]
fn add_text() {
    assert_eq!(BinaryOp::Pow.right_assoc(), true);
}