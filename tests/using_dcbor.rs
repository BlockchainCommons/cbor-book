use anyhow::Result;
use std::{collections::HashMap, vec};

// ANCHOR: first

// This is all you need to import to use the library.
use dcbor::prelude::*;

// ANCHOR_END: first

#[rustfmt::skip]
// ANCHOR: second

pub fn main() {
    // Encode the integer 42
    let i = 42;
    let cbor: CBOR = i.to_cbor();
    // The CBOR type above here for clarity, can be inferred

    // Check the diagnostic representation
    assert_eq!(cbor.diagnostic(), "42");

    // Check the hex representation
    assert_eq!(cbor.hex(), "1a002a");

    // Check the CBOR data
    assert_eq!(cbor.to_cbor_data(), vec![0x1a, 0x00, 0x2a]);
}

// ANCHOR_END: second

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

let hex = cbor.hex_annotated();
let expected_hex = r#"

85      # array(5)
    01  # unsigned(1)
    02  # unsigned(2)
    03  # unsigned(3)
    04  # unsigned(4)
    05  # unsigned(5)

"#.trim();

assert_eq!(hex, expected_hex);

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

let hex = cbor.hex_annotated();
let expected_hex = r#"

45              # bytes(5)
    0102030405

"#.trim();

assert_eq!(hex, expected_hex);

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

let diagnostic = cbor.diagnostic();
let expected_diagnostic = "[true, false, null]";
assert_eq!(diagnostic, expected_diagnostic);

let hex = cbor.hex_annotated();
let expected_hex = r#"

83      # array(3)
    f5  # true
    f4  # false
    f6  # null

"#.trim();

assert_eq!(hex, expected_hex);

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
let hex = hex::encode(&data);
let expected_hex = "a266636f6c6f7273836372656465677265656e64626c756567616e696d616c73836363617463646f6765686f727365";
assert_eq!(hex, expected_hex);

// Deserialize the data back into a CBOR object
let cbor2: CBOR = CBOR::try_from_data(data)?;

// Convert the CBOR object back into a HashMap
let h2: HashMap<String, Vec<String>> = cbor2.try_into()?;

// Check that the original and deserialized HashMaps are equal
assert_eq!(h, h2);

// ANCHOR_END: test_11
Ok(())
}


#[test]
#[rustfmt::skip]
fn test_12() -> Result<()> {
// ANCHOR: test_12

// Create a HashMap with integer keys and Vec<String> values
let mut h: HashMap<usize, Vec<String>> = HashMap::new();
h.insert(1, ["cat", "dog", "horse"].map(str::to_string).to_vec());
h.insert(2, ["red", "green", "blue"].map(str::to_string).to_vec());

// Convert the HashMap to a CBOR object
let cbor = h.to_cbor();

// Check the representation in CBOR diagnostic notation
let diagnostic = cbor.diagnostic_flat();
let expected_diagnostic = r#"

{1: ["cat", "dog", "horse"], 2: ["red", "green", "blue"]}

"#.trim();
assert_eq!(diagnostic, expected_diagnostic);

// Convert the CBOR object back into a HashMap
let h2: HashMap<usize, Vec<String>> = cbor.try_into()?;

// Check that the original and deserialized HashMaps are equal
assert_eq!(h, h2);

// ANCHOR_END: test_12
Ok(())
}


#[test]
#[rustfmt::skip]
fn test_13() -> Result<()> {
// ANCHOR: test_13

// Create a HashMap with CBOR for its keys and values
let mut h: HashMap<CBOR, CBOR> = HashMap::new();
h.insert(1.into(), vec![CBOR::from("cat"), "dog".into(), "horse".into()].into());
h.insert(2.into(), vec![CBOR::from("red"), "green".into(), "blue".into()].into());

// Convert the HashMap to a CBOR object
let cbor = h.to_cbor();

// Check the representation in CBOR diagnostic notation
let diagnostic = cbor.diagnostic_flat();
let expected_diagnostic = r#"

{1: ["cat", "dog", "horse"], 2: ["red", "green", "blue"]}

"#.trim();
assert_eq!(diagnostic, expected_diagnostic);

// Convert the CBOR object back into a HashMap
let h2: HashMap<CBOR, CBOR> = cbor.try_into()?;

// Check that the original and deserialized HashMaps are equal
assert_eq!(h, h2);

// ANCHOR_END: test_13
Ok(())
}

#[test]
#[rustfmt::skip]
fn test_14() -> Result<()> {
// ANCHOR: test_14

// number to CBOR
let n = 10;
let cbor = n.to_cbor();
// CBOR to number
assert_eq!(i32::try_from(cbor.clone())?, n);
assert_eq!(f64::try_from(cbor)?, n as f64);

// bool to CBOR
let b = true;
let cbor = b.to_cbor();
// CBOR to bool
assert_eq!(bool::try_from(cbor.clone())?, b);
assert_eq!(cbor.try_into_bool()?, b);

// null to CBOR
let n = CBOR::null();
let cbor = n.to_cbor();
// CBOR to null
assert_eq!(CBOR::try_from(cbor.clone())?, n);
assert!(cbor.is_null());

// bstr to CBOR
let v = vec![1, 2, 3, 4, 5];
let b = ByteString::from(v.clone());
let cbor = b.to_cbor();
let cbor2 = CBOR::to_byte_string(v.clone());
assert_eq!(cbor, cbor2);
// CBOR to bstr
assert_eq!(ByteString::try_from(cbor.clone())?, b);
let array: Vec<u8> = cbor.try_into_byte_string()?;
assert_eq!(array, v);

// tstr to CBOR
let t = "Hello";
let cbor = t.to_cbor();
// CBOR to tstr
assert_eq!(String::try_from(cbor.clone())?, t);
assert_eq!(cbor.try_into_text()?, t);

// array to CBOR
let a = vec![1, 2, 3];
let cbor = a.to_cbor();
// CBOR to homogenous array
let b = Vec::<i32>::try_from(cbor.clone())?;
assert_eq!(b, a);
// CBOR to heterogeneous array
let b: Vec<CBOR> = cbor.clone().try_into_array()?;
let b: Vec<i32> = b.into_iter()
    .map(|x| i32::try_from(x).map_err(Into::into))
    .collect::<Result<_>>()?;
assert_eq!(b, a);
let b: Vec<CBOR> = cbor.try_into_array()?;
let b: Vec<i32> = vec![
    b[0].clone().try_into()?,
    b[1].clone().try_into()?,
    b[2].clone().try_into()?,
];
assert_eq!(b, a);

// map to CBOR
let mut m: HashMap<String, i32> = HashMap::new();
m.insert("a".into(), 1);
m.insert("b".into(), 2);
let cbor = m.to_cbor();
// CBOR to homogenous map
let m2: HashMap<String, i32> = cbor.clone().try_into()?;
assert_eq!(m, m2);
// CBOR to heterogeneous map
let m2: dcbor::Map = cbor.clone().try_into_map()?;
let m2: HashMap<String, i32> = m2.iter()
    .map(|(k, v)| {
        let k = String::try_from(k.clone()).map_err(anyhow::Error::from)?;
        let v = i32::try_from(v.clone()).map_err(anyhow::Error::from)?;
        Ok((k, v))
    })
    .collect::<Result<_>>()?;
assert_eq!(m, m2);
let m2: dcbor::Map = cbor.try_into_map()?;
let a: i32 = m2.extract("a")?;
assert_eq!(a, 1);
let b: i32 = m2.extract("b")?;
assert_eq!(b, 2);

// tagged to CBOR
let t = CBOR::to_tagged_value(999, "Hello");
let cbor = t.to_cbor();
// CBOR to tagged
let t2: (Tag, CBOR) = cbor.try_into_tagged_value()?;
assert_eq!(Tag::from(999), t2.0);
assert_eq!(String::try_from(t2.1.clone())?, "Hello");
// tagged (with name) to CBOR
let named_tag = Tag::new(999, "my-tag");
let t = CBOR::to_tagged_value(named_tag.clone(), "Hello");
let cbor = t.to_cbor();
// CBOR to tagged
let t2: (Tag, CBOR) = cbor.clone().try_into_tagged_value()?;
assert_eq!(named_tag, t2.0);
assert_eq!(String::try_from(t2.1.clone())?, "Hello");
// Expecting a specific tag
let t2: CBOR = cbor.clone().try_into_expected_tagged_value(named_tag.clone())?;
assert_eq!(String::try_from(t2.clone())?, "Hello");

// Registering a tag for diagnostic annotation
with_tags_mut!(|tags: &mut TagsStore| {
    tags.insert(named_tag.clone());
});
assert_eq!(cbor.diagnostic_annotated(), r#"999("Hello")   / my-tag /"#);

// ANCHOR_END: test_14
Ok(())
}
