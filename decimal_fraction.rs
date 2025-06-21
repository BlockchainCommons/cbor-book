use dcbor::prelude::*;

use crate::tags::TAG_DECIMAL_FRACTION;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// ANCHOR: example_6
pub struct DecimalFraction {
    pub exponent: i8,
    pub mantissa: i64,
}
// ANCHOR_END: example_6

impl DecimalFraction {
    /// Create a new `DecimalFraction` from raw parts.
    pub fn new(exponent: i8, mantissa: i64) -> Self {
        Self { exponent, mantissa }
    }

    /// Convert back to `f64`. May lose precision on large exponents.
    pub fn to_f64(self) -> f64 {
        (self.mantissa as f64) * (10f64).powi(self.exponent as i32)
    }
}

impl std::fmt::Display for DecimalFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.mantissa == 0 {
            return write!(f, "0");
        }

        let abs_value = self.mantissa.abs();
        let is_negative = self.mantissa < 0;
        let prefix = if is_negative { "-" } else { "" };

        if self.exponent >= 0 {
            // For positive exponent, add zeros after the number
            write!(f, "{}{}{}", prefix, abs_value, "0".repeat(self.exponent as usize))
        } else {
            // For negative exponent, insert decimal point
            let abs_exp = -self.exponent as usize;
            let value_str = abs_value.to_string();

            if value_str.len() <= abs_exp {
                // Decimal point at the beginning with possible leading zeros
                let padding = abs_exp - value_str.len();
                write!(f, "{}0.{}{}", prefix, "0".repeat(padding), value_str)
            } else {
                // Insert decimal point within the number
                let decimal_pos = value_str.len() - abs_exp;
                let (integer_part, fractional_part) = value_str.split_at(decimal_pos);
                write!(f, "{}{}.{}", prefix, integer_part, fractional_part)
            }
        }
    }
}

// ANCHOR: example_7
impl From<DecimalFraction> for CBOR {
    fn from(value: DecimalFraction) -> Self {
        // Compose the two-element array
        let v = vec![value.exponent as i64, value.mantissa].to_cbor();

        // Return the tagged array
        CBOR::to_tagged_value(TAG_DECIMAL_FRACTION, v)
    }
}
// ANCHOR_END: example_7

// ANCHOR: example_9
impl TryFrom<CBOR> for DecimalFraction {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        // Decode the tagged array
        let item = cbor.try_into_expected_tagged_value(TAG_DECIMAL_FRACTION)?;

        // Convert the item to an array
        let arr = item.try_into_array()?;

        // Validate the length of the array
        if arr.len() != 2 {
            return Err("Expected a two-element array".into());
        }

        // Extract the exponent and mantissa
        let exponent: i8 = arr[0].clone().try_into()?;
        let mantissa: i64 = arr[1].clone().try_into()?;

        // Return the DecimalFraction
        Ok(DecimalFraction::new(exponent, mantissa))
    }
}
// ANCHOR_END: example_9

#[test]
fn decimal_fraction() {
    let a = DecimalFraction::new(-1, 11);
    assert_eq!(a.mantissa, 11);
    assert_eq!(a.exponent, -1);
    assert!((a.to_f64() - 1.1).abs() < f64::EPSILON);

    let b = DecimalFraction::new(-2, 101);
    assert_eq!(b.mantissa, 101);
    assert_eq!(b.exponent, -2);
    assert!((b.to_f64() - 1.01).abs() < f64::EPSILON);
}

#[test]
fn decimal_fraction_display() {
    // Test zero
    let zero = DecimalFraction::new(0, 0);
    assert_eq!(zero.to_string(), "0");

    // Test positive value with zero exponent
    let simple = DecimalFraction::new(0, 42);
    assert_eq!(simple.to_string(), "42");

    // Test positive values with positive exponent
    let pos_exp1 = DecimalFraction::new(2, 5);
    assert_eq!(pos_exp1.to_string(), "500");

    let pos_exp2 = DecimalFraction::new(3, 123);
    assert_eq!(pos_exp2.to_string(), "123000");

    // Test negative values with positive exponent
    let neg_pos_exp = DecimalFraction::new(1, -42);
    assert_eq!(neg_pos_exp.to_string(), "-420");

    // Test positive values with negative exponent
    let pos_neg_exp1 = DecimalFraction::new(-2, 123);
    assert_eq!(pos_neg_exp1.to_string(), "1.23");

    let pos_neg_exp2 = DecimalFraction::new(-1, 5);
    assert_eq!(pos_neg_exp2.to_string(), "0.5");

    let pos_neg_exp3 = DecimalFraction::new(-3, 5);
    assert_eq!(pos_neg_exp3.to_string(), "0.005");

    // Test negative values with negative exponent
    let neg_neg_exp1 = DecimalFraction::new(-2, -123);
    assert_eq!(neg_neg_exp1.to_string(), "-1.23");

    let neg_neg_exp2 = DecimalFraction::new(-3, -5);
    assert_eq!(neg_neg_exp2.to_string(), "-0.005");

    // Test boundary cases
    let boundary1 = DecimalFraction::new(-9, 123456789);
    assert_eq!(boundary1.to_string(), "0.123456789");

    let boundary2 = DecimalFraction::new(-1, 1);
    assert_eq!(boundary2.to_string(), "0.1");
}
