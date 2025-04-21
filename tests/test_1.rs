use anyhow::Result;
use std::{collections::HashMap, vec};

// ANCHOR: first
// This is all you need to import to use the library.
use dcbor::prelude::*;

#[rustfmt::skip]
pub fn main() {
    // Encode the integer 42
    let i = 42;
    let cbor = i.to_cbor();

    // Check the diagnostic representation
    assert_eq!(cbor.diagnostic(), "42");

    // Check the hex representation
    assert_eq!(cbor.hex(), "1a002a");

    // Check the CBOR data
    assert_eq!(cbor.to_cbor_data(), vec![0x1a, 0x00, 0x2a]);
}
// ANCHOR_END: first

#[test]
#[rustfmt::skip]
fn test_2() -> Result<()> {
// ANCHOR: test_2
let a = 42;
let cbor = a.to_cbor();
let b = i32::try_from(cbor)?;
assert_eq!(a, b);
// ANCHOR_END: test_2
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_3() -> Result<()> {
// ANCHOR: test_3
let a = 42;
let cbor = a.to_cbor();

// Decode as a u8
let b = u8::try_from(cbor.clone())?;
assert_eq!(a as u8, b);

// Decode as an f64
let c = f64::try_from(cbor)?;
assert_eq!(a as f64, c);
// ANCHOR_END: test_3
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_4() -> Result<()> {
// ANCHOR: test_4
let a = 1.23456;
let cbor = a.to_cbor();

// Decode as an f64
let b = f64::try_from(cbor.clone())?;
assert_eq!(a, b);

// Cannot decode as a i32
assert!(u8::try_from(cbor).is_err());
// ANCHOR_END: test_4
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_5() -> Result<()> {
// ANCHOR: test_5
let a = "Hello, dCBOR!";
let cbor = a.to_cbor();

// Decode as an f64 fails
assert!(f64::try_from(cbor.clone()).is_err());

// Decode as a String succeeds
let b = String::try_from(cbor)?;
assert_eq!(a, b);
// ANCHOR_END: test_5
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_6() -> Result<()> {
// ANCHOR: test_6
// Encode a vector of 8-bit unsigned integers
let a: Vec<u8> = vec![1, 2, 3, 4, 5];
let cbor = a.to_cbor();

// Decode as Vec of a compatible type: 32-bit signed integers
let b: Vec<i32> = Vec::try_from(cbor.clone())?;
assert_eq!(b, vec![1, 2, 3, 4, 5]);
// ANCHOR_END: test_6
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_7() -> Result<()> {
// ANCHOR: test_7
// Encode a vector of 8-bit unsigned integers
let a: Vec<u8> = vec![1, 2, 3, 4, 5];
let cbor = a.to_cbor();

let expected = r#"

85      # array(5)
    01  # unsigned(1)
    02  # unsigned(2)
    03  # unsigned(3)
    04  # unsigned(4)
    05  # unsigned(5)

"#.trim();

assert_eq!(cbor.hex_annotated(), expected);
// ANCHOR_END: test_7
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_8() -> Result<()> {
// ANCHOR: test_8
// Encode a vector of 8-bit unsigned integers
let a = vec![1, 2, 3, 4, 5];
let byte_string = CBOR::to_byte_string(a);
let cbor = byte_string.to_cbor();

let expected = r#"

45              # bytes(5)
    0102030405

"#.trim();

assert_eq!(cbor.hex_annotated(), expected);

let b: Vec<u8> = ByteString::try_from(cbor)?.into();
assert_eq!(b, vec![1, 2, 3, 4, 5]);
// ANCHOR_END: test_8
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_9() -> Result<()> {
// ANCHOR: test_9
let v: Vec<CBOR> = vec![
    true.into(),
    false.into(),
    CBOR::null(),
];
let cbor = v.to_cbor();

let expected_diagnostic = "[true, false, null]";

assert_eq!(cbor.diagnostic(), expected_diagnostic);

let expected_hex = r#"

83      # array(3)
    f5  # true
    f4  # false
    f6  # null

"#.trim();

assert_eq!(cbor.hex_annotated(), expected_hex);
// ANCHOR_END: test_9
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_10() -> Result<()> {
// ANCHOR: test_10
// Compose an array of CBOR values
let v: Vec<CBOR> = vec![
    true.into(),
    false.into(),
    CBOR::null(),
];
// Convert the array to a single CBOR object, which would
// be serialized to CBOR data or recovered from it.
let cbor: CBOR = v.to_cbor();

// Recover the array from the CBOR object
let v2: Vec<CBOR> = cbor.try_into_array()?;

// Check the length of the array
assert_eq!(v2.len(), 3);

// For the first value (`true`), extract it so it could be saved for later.
let t = v2[0].clone().try_into_bool()?;
assert!(t);

// For the second value (`false`), just assert that it is false.
assert!(v2[1].is_false());

// For the third value (`null`), assert that it is null.
assert!(v2[2].is_null());

// ANCHOR_END: test_10
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_11() -> Result<()> {
// ANCHOR: test_11
// Create a HashMap with String keys and Vec<String> values
let mut h: HashMap<String, Vec<String>> = HashMap::new();
h.insert("animals".into(), vec!("cat".into(), "dog".into(), "horse".into()));
h.insert("colors".into(), vec!["red".into(), "green".into(), "blue".into()]);

// Convert the HashMap to a CBOR object
let cbor = h.to_cbor();

// Check the representation in CBOR diagnostic notation
let diagnostic = cbor.diagnostic();
let expected_diagnostic = r#"

{
    "colors":
    ["red", "green", "blue"],
    "animals":
    ["cat", "dog", "horse"]
}

"#.trim();
assert_eq!(diagnostic, expected_diagnostic);

// Serialize the CBOR to binary data
let data: Vec<u8> = cbor.to_cbor_data();

// Check the hex representation of the serialized data
assert_eq!(hex::encode(&data),
    "a266636f6c6f7273836372656465677265656e64626c756567616e696d616c73836363617463646f6765686f727365",
);

// Deserialize the data back into a CBOR object
let cbor2: CBOR = CBOR::try_from_data(data)?;

// Convert the CBOR object back into a HashMap
let h2: HashMap<String, Vec<String>> = cbor2.try_into()?;

// Check that the original and deserialized HashMaps are equal
assert_eq!(h, h2);
// ANCHOR_END: test_11
Ok(())
}
