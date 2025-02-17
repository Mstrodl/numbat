use num_traits::Pow;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)] // TODO: we probably want to remove 'Copy' once we move to a more sophisticated numerical type
pub struct Number(pub f64);

impl Eq for Number {}

impl Number {
    pub fn from_f64(n: f64) -> Self {
        Number(n)
    }

    pub fn to_f64(self) -> f64 {
        let Number(n) = self;
        n
    }

    pub fn pow(self, other: &Number) -> Self {
        Number::from_f64(self.to_f64().pow(other.to_f64()))
    }

    fn is_integer(self) -> bool {
        self.0.trunc() == self.0
    }

    pub fn pretty_print(self) -> String {
        let number = self.0;

        // 64-bit floats can accurately represent integers up to 2^52 [1].
        //
        // [1] https://stackoverflow.com/a/43656339
        //
        if self.is_integer() && self.0.abs() < 1e15 {
            format!("{number}")
        } else {
            use pretty_dtoa::{dtoa, FmtFloatConfig};

            let config = FmtFloatConfig::default()
                .max_significant_digits(6)
                .add_point_zero(false)
                .lower_e_break(-6)
                .round();

            let formatted_number = dtoa(number, config);

            if formatted_number.contains('.') && !formatted_number.contains('e') {
                let formatted_number = formatted_number.trim_end_matches('0');
                if formatted_number.ends_with('.') {
                    format!("{}0", formatted_number)
                } else {
                    formatted_number.to_string()
                }
            } else {
                formatted_number
            }
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}

impl std::ops::Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl std::ops::Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}

impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

impl std::iter::Product for Number {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Number::from_f64(1.0), |acc, n| acc * n)
    }
}

#[test]
fn test_pretty_print() {
    assert_eq!(Number::from_f64(1.).pretty_print(), "1");
    assert_eq!(Number::from_f64(100.).pretty_print(), "100");
    assert_eq!(Number::from_f64(1.234).pretty_print(), "1.234");
    assert_eq!(Number::from_f64(1.234e50).pretty_print(), "1.234e50");
    assert_eq!(Number::from_f64(-1.234e50).pretty_print(), "-1.234e50");
    assert_eq!(Number::from_f64(1.234e-50).pretty_print(), "1.234e-50");
    assert_eq!(Number::from_f64(-1.234e-50).pretty_print(), "-1.234e-50");

    assert_eq!(Number::from_f64(1234567890.).pretty_print(), "1234567890");
    assert_eq!(
        Number::from_f64(1234567890000000.).pretty_print(),
        "1.23457e15"
    );

    assert_eq!(Number::from_f64(1.23456789).pretty_print(), "1.23457");
    assert_eq!(
        Number::from_f64(1234567890000.1).pretty_print(),
        "1.23457e12"
    );

    assert_eq!(Number::from_f64(100.00001).pretty_print(), "100.0");

    assert_eq!(Number::from_f64(0.00001).pretty_print(), "0.00001");
    assert_eq!(Number::from_f64(0.000001).pretty_print(), "0.000001");
    assert_eq!(Number::from_f64(0.0000001).pretty_print(), "1.0e-7");
}
