# A Practical Introduction to CBOR

## From Comparison to Construction

In the previous chapter, we explored the diverse landscape of binary serialization formats, comparing CBOR to its contemporaries like BSON, Protocol Buffers, MessagePack, and Avro. We saw how each format emerged from different needs and design philosophies, resulting in distinct trade-offs between schema requirements, performance, compactness, and features like schema evolution. CBOR, standardized by the IETF as [RFC 8949](https://datatracker.ietf.org/doc/html/rfc8949), carved out its niche by providing a binary encoding based on the familiar JSON data model, optimized for efficiency (especially in constrained environments like IoT), extensibility, and standardization within internet protocols.

Having understood _why_ CBOR exists and how it relates to other formats, we now shift our focus to _how_ it works. This chapter provides a practical introduction to the core mechanics of CBOR encoding. The goal is not to replicate the exhaustive detail of RFC 8949, but rather to quickly equip engineers with a solid working understanding of how fundamental data types are represented in CBOR.

> **⚠️ NOTE:** Wherever this book may conflict with RFC 8949, the RFC is authoritative. This book is intended to be a practical guide, not a definitive reference. We will also use the term "CBOR" interchangeably to refer to both the encoding and the data model, unless otherwise specified.

We will progressively build up understanding by examining common data structures, comparing their representation in:

- **JSON:** The familiar text-based format.
- [**CBOR Diagnostic Notation:**](https://datatracker.ietf.org/doc/draft-ietf-cbor-edn-literals/) A human-readable text format, similar to JSON but extended for CBOR's features, used for documentation and debugging.
- **Hexadecimal CBOR:** The actual binary representation shown as hexadecimal bytes, which is how CBOR data is transmitted or stored.

We will focus on the most common, definite-length encodings and the concept of "preferred serialization" – using the shortest possible form where choices exist. Advanced topics such as semantic tags (Major Type 6), indefinite-length encoding, full deterministic encoding rules (beyond preferred serialization), schema definition with CDDL, and CBOR sequences will be introduced in later chapters. By the end of this chapter, you should be able to look at simple CBOR byte sequences and understand the data they represent.

> **✅ TIP:** The [CBOR Playground](https://cbor.me/) is an excellent tool if you would like to follow along with the examples, converting CBOR Diagnostic Notation to binary and back.

## The Core Encoding: Major Types and Additional Information

At the heart of CBOR's encoding lies a simple yet powerful structure contained within the first byte (and potentially subsequent bytes) of any data item. This initial byte conveys two crucial pieces of information:

```
┌────┬────┬────┬────┬────┬────┬────┬────┐
│  7 │  6 │  5 │  4 │  3 │  2 │  1 │  0 │
├────┴────┴────┼────┴────┴────┴────┴────┤
│  MAJOR TYPE  │ ADDITIONAL INFORMATION │
└──────────────┴────────────────────────┘
```

1. **Major Type (MT):** The high-order 3 bits (bits 5, 6, and 7) define the general category of the data item. There are 8 major types (0 through 7).
2. **Additional Information (AI):** The low-order 5 bits (bits 0 through 4) provide specific details about the data item, whose meaning depends on the Major Type. This can range from encoding the entire value directly (for small integers or simple constants) to indicating the length of subsequent data or specifying the precision of a floating-point number.

This initial byte structure allows a CBOR decoder to immediately understand the fundamental type and size characteristics of the data item it is encountering, enabling efficient parsing without requiring a predefined schema. All multi-byte numerical values in CBOR are encoded in network byte order (big-endian).

Let's break down the Major Types and see how the Additional Information works for each:

| Major Type | Bits (MT)  | Meaning                         | Notes                                           |
|------------|------------|---------------------------------|-------------------------------------------------|
| 0          | `000`      | Unsigned Integer                | Values from `0` to `2⁶⁴−1`                      |
| 1          | `001`      | Negative Integer                | Encodes `-1 - n` where `n` is the encoded value |
| 2          | `010`      | Byte String                     | Sequence of raw bytes                           |
| 3          | `011`      | Text String                     | UTF-8 encoded string                            |
| 4          | `100`      | Array                           | Ordered list of data items                      |
| 5          | `101`      | Map                             | Pairs of keys and values                        |
| 6          | `110`      | Tag                             | Semantic qualifier for the following item       |
| 7          | `111`      | Simple Values / Floating-Point  | Booleans, null, undefined, floats, etc.         |

The Additional Information values (0-31) modify the meaning of the Major Type:

| AI Value | Bits (AI) | Meaning                                                   |
|----------|-----------|-----------------------------------------------------------|
| 0–23     | `00000`–`10111` | Value or length is encoded directly (literal value) |
| 24       | `11000`   | Next 1 byte contains the value or length (uint8)          |
| 25       | `11001`   | Next 2 bytes contain the value or length (uint16)         |
| 26       | `11010`   | Next 4 bytes contain the value or length (uint32)         |
| 27       | `11011`   | Next 8 bytes contain the value or length (uint64)         |
| 28–30    | `11100`–`11110` | Reserved for future use                             |
| 31       | `11111`   | Indefinite-length indicator or special “break”            |

AI values 0-27 are used for encoding the length of the data item or the value itself, with 24-27 called the `1+1`, `1+2`, `1+4`, and `1+8` encodings, respectively. The AI value 31 is used for indefinite-length items, which we will cover in a later chapter.

```
    ┌──────┐
1   │ 0-23 │
    └──────┘
    ┌────┐┌────┐
1+1 │ 24 ││    │
    └────┘└────┘
    ┌────┐┌────┬────┐
1+2 │ 25 ││    │    │
    └────┘└────┴────┘
    ┌────┐┌────┬────┬────┬────┐
1+4 │ 26 ││    │    │    │    │
    └────┘└────┴────┴────┴────┘
    ┌────┐┌────┬────┬────┬────┬────┬────┬────┬────┐
1+8 │ 27 ││    │    │    │    │    │    │    │    │
    └────┘└────┴────┴────┴────┴────┴────┴────┴────┘
```

Understanding this MT/AI structure is the key to decoding CBOR. We will now see it in action as we explore specific data types. [An appendix](../appendices/cbor_header_bytes.md) contains a table of all 256 possible Major Type and Additional Information combinations.

## Simple Scalar Types: Integers, Booleans, and Null

Let's start with the simplest data types common to JSON and CBOR: integers, booleans, and null.

### Integers (Major Types 0 & 1)

CBOR distinguishes between unsigned integers (Major Type 0) and negative integers (Major Type 1). The Additional Information determines how the integer's value (or _argument_) is encoded.

- **Small Integers (0-23):** If the unsigned integer is between 0 and 23 inclusive, it's encoded directly in the Additional Information bits of the initial byte (Major Type 0).
- **Larger Integers:** For values 24 or greater, the Additional Information takes the value 24, 25, 26, or 27, indicating that the actual integer value follows in the next 1, 2, 4, or 8 bytes, respectively, in network byte order (big-endian).
- **Negative Integers:** Encoded using Major Type 1. The value encoded is `−1 − argument`. So, an argument of 0 represents the integer `-1`, an argument of 9 represents `-10`, and so on. The argument itself is encoded using the same rules as unsigned integers (AI 0-23 for arguments 0-23, AI 24-27 for larger arguments).

Preferred Serialization: CBOR allows multiple ways to encode the same number (e.g., the number `10` could theoretically be encoded using 1, 2, 4, or 8 bytes following an initial byte with AI 24, 25, 26, or 27). However, the standard strongly recommends *preferred serialization*, which means always using the shortest possible form. This avoids ambiguity and unnecessary padding. For non-negative integers, this means:

| Value Range           | AI Value | Bytes Used After Initial Byte | Total Encoding Size |
|-----------------------|----------|-------------------------------|---------------------|
| 0–23                  | 0–23     | 0                             | 1 byte              |
| 24–255                | 24       | 1                             | 2 bytes             |
| 256–65,535            | 25       | 2                             | 3 bytes             |
| 65,536–4,294,967,295  | 26       | 4                             | 5 bytes             |
| 4,294,967,296–2⁶⁴−1   | 27       | 8                             | 9 bytes             |

The same principle applies to the argument for negative integers.

**Examples (Preferred Serialization):**

| JSON     | CBOR Diagnostic | CBOR Hex       | MT | AI  | Explanation                                          |
|----------|------------------|----------------|----|-----|------------------------------------------------------|
| `0`      | `0`              | `00`           | 0  | 0   | Value 0 directly encoded                            |
| `10`     | `10`             | `0a`           | 0  | 10  | Value 10 directly encoded                           |
| `23`     | `23`             | `17`           | 0  | 23  | Value 23 directly encoded                           |
| `24`     | `24`             | `18 18`        | 0  | 24  | Value in next byte; `0x18` = 24                     |
| `100`    | `100`            | `18 64`        | 0  | 24  | Value in next byte; `0x64` = 100                    |
| `1000`   | `1000`           | `19 03e8`      | 0  | 25  | Value in next 2 bytes; `0x03e8` = 1000              |
| `1000000`| `1000000`        | `1a 000f4240`  | 0  | 26  | Value in next 4 bytes; `0x000f4240` = 1,000,000     |
| `-1`     | `-1`             | `20`           | 1  | 0   | -1 = -1 - 0                                          |
| `-10`    | `-10`            | `29`           | 1  | 9   | -10 = -1 - 9                                         |
| `-100`   | `-100`           | `38 63`        | 1  | 24  | Argument in next byte; `0x63` = 99 → -1 - 99 = -100 |
| `-1000`  | `-1000`          | `39 03e7`      | 1  | 25  | Argument in next 2 bytes; `0x03e7` = 999 → -1000    |

### Booleans and Null (Major Type 7)

CBOR uses Major Type 7 for various simple values and floating-point numbers. The boolean values `true` and `false`, and the `null` value, have specific, fixed Additional Information values.

| JSON     | CBOR Diagnostic  | CBOR Hex | MT | AI  | Explanation           |
|----------|------------------|----------|----|-----|-----------------------|
| `false`  | `false`          | `f4`     | 7  | 20  | Simple value: `false` |
| `true`   | `true`           | `f5`     | 7  | 21  | Simple value: `true`  |
| `null`   | `null`           | `f6`     | 7  | 22  | Simple value: `null`  |

CBOR also defines an `undefined` simple value (`f7`, MT 7, AI 23), which doesn't have a direct equivalent in standard JSON but may be useful in certain protocols.

## Strings: Bytes and Text

CBOR has distinct types for byte strings (arbitrary sequences of bytes) and text strings (sequences of Unicode characters encoded as UTF-8). This is a key advantage over JSON, which lacks native binary support and typically requires base64 encoding for binary data.

### Byte Strings (Major Type 2)

Byte strings use Major Type 2. The Additional Information encodes the length of the string in bytes, following the same rules as unsigned integers (AI 0-23 for lengths 0-23, AI 24-27 + subsequent bytes for longer lengths). The raw bytes of the string immediately follow the initial byte(s).

### Examples: Definite Length Byte Strings

In CBOR diagnostic notation, byte strings are represented using hexadecimal encoding prefixed with `h` and enclosed in single quotes.

| Description                 | CBOR Diagnostic  | CBOR Hex         | MT | AI  | Explanation                                             |
|-----------------------------|------------------|------------------|----|-----|---------------------------------------------------------|
| Empty byte string           | `h''`            | `40`             | 2  | 0   | Length 0 bytes                                          |
| Bytes `0x01, 0x02, 0x03`    | `h'010203'`      | `43 010203`      | 2  | 3   | Length 3 bytes; followed by `01 02 03`                  |
| 24 bytes (e.g., all `0x00`) | `h'…'`           | `58 18 …`        | 2  | 24  | Length in next byte; `0x18` = 24; followed by 24 bytes  |

### Examples: Definite Length Text Strings

Text strings use Major Type 3 and are explicitly defined as UTF-8 encoded Unicode strings. The Additional Information (AI) specifies the length in *bytes* of the UTF-8 encoding, *not* the number of Unicode characters, which can (and often are) different. In diagnostic notation, text strings are enclosed in double quotes (like JSON).

| JSON     | CBOR Diagnostic | CBOR Hex            | MT | AI | Explanation                                                        |
|----------|------------------|---------------------|----|----|--------------------------------------------------------------------|
| `""`     | `""`             | `60`                | 3  | 0  | Empty string; length 0                                             |
| `"a"`    | `"a"`            | `61 61`             | 3  | 1  | Length 1 byte; `0x61` = `'a'`                                      |
| `"hello"`| `"hello"`        | `65 68656c6c6f`     | 3  | 5  | Length 5 bytes; `68 65 6c 6c 6f` = `'hello'`                       |
| `"IETF"` | `"IETF"`         | `64 49455446`       | 3  | 4  | Length 4 bytes; `49 45 54 46` = `'IETF'`                           |
| `"ü"`    | `"ü"`            | `62 c3bc`           | 3  | 2  | Length 2 bytes; `c3 bc` is UTF-8 for `'ü'`                         |
| `"你好"` | `"你好"`         | `66 e4bda0e5a5bd`   | 3  | 6  | Length 6 bytes; `e4 bd a0 e5 a5 bd` is UTF-8 for `'你好'`           |

> **⚠️ NOTE:** that CBOR does not perform string escaping like JSON does (e.g., for quotes or backslashes). Since the length is provided upfront, the decoder knows exactly how many bytes constitute the string content. So the string `"Hello"`, *including* the quotes is seven bytes long, and the CBOR encoding would be eight bytes:

```
67               # Text(7 bytes)
  2248656C6C6F22 # "Hello"
```

> If you use the CBOR Playground to convert this to Diagnostic Notation, you'll get:

```
"\"Hello\""
```

> So backslash escapes are part of CBOR Diagnostic Notation, but *not* part of the CBOR encoding itself.

## Collections: Arrays and Maps

CBOR supports ordered sequences of items (arrays) and unordered key-value pairs (maps), mirroring JSON's structures but with some key differences. This section focuses on definite-length collections, where the number of elements or pairs is known upfront.

### Arrays (Major Type 4)

Arrays use Major Type 4. The Additional Information encodes the number of data items (elements) contained within the array, using the same encoding rules as unsigned integers (AI 0-23 for counts 0-23, AI 24-27 + subsequent bytes for larger counts). The encoded data items follow the initial byte(s) in sequence.

Like JSON, CBOR Diagnostic Notation uses square brackets `[]` with comma-separated elements.

### Examples: Definite Length Arrays

Arrays in CBOR use Major Type 4. The Additional Information (AI) specifies the number of elements in the array. The elements are encoded sequentially after the initial byte.

| JSON            | CBOR Diagnostic | CBOR Hex         | MT | AI | Explanation                                                         |
|-----------------|-----------------|------------------|----|----|---------------------------------------------------------------------|
| `[]`            | `[]`            | `80`             | 4  | 0  | Array with 0 elements                                               |
| `[1, 2, 3]`     | `[1, 2, 3]`     | `83 01 02 03`    | 4  | 3  | Array with 3 elements; `01`, `02`, `03` encode integers 1, 2, and 3 |
| `[true, null]`  | `[true, null]`  | `82 f5 f6`       | 4  | 2  | Array with 2 elements; `f5` = true, `f6` = null                     |
| _no equivalent_ | `["a", h'01']`  | `82 61 61 41 01` | 4  | 2  | Array with 2 elements; `61 61` = "a", `41 01` = byte string `h'01'` |

### Maps (Major Type 5)

Maps (also known variously as *dictionaries* or *associative arrays*) use Major Type 5. The Additional Information encodes the number of _pairs_ in the map (not the total number of keys and values). Again, the encoding follows the rules for unsigned integers. The key-value pairs follow the initial byte(s), with each key immediately followed by its corresponding value (key1, value1, key2, value2,...).

A significant difference from JSON is that CBOR map keys can be _any_ CBOR data type (integers, strings, arrays, etc.), not just text strings.

### Examples: Definite Length Maps

CBOR maps use Major Type 5. The Additional Information (AI) specifies the number of key-value pairs. Keys and values follow in alternating sequence. Diagnostic notation uses curly braces `{}` with comma-separated `key: value` pairs.

| JSON               | CBOR Diagnostic        | CBOR Hex                       | MT | AI | Explanation                                                           |
|--------------------|------------------------|--------------------------------|----|----|-----------------------------------------------------------------------|
| `{}`               | `{}`                   | `a0`                           | 5  | 0  | Map with 0 key-value pairs                                            |
| `{"a": 1}`         | `{"a": 1}`             | `a1 61 61 01`                  | 5  | 1  | 1 pair: key `"a"` (`61 61`), value `1` (`01`)                         |
| `{"a": 1, "b": 2}` | `{"a": 1, "b": 2}`     | `a2 61 61 01 61 62 02`         | 5  | 2  | 2 pairs: `"a"`→`1`, `"b"`→`2`; encoded in sequence                    |
| _no equivalent_    | `{1: "one", 2: "two"}` | `a2 01 63 6f6e65 02 63 74776f` | 5  | 2  | 2 pairs: `1`→`"one"`, `2`→`"two"`; strings encoded as `63` (length 3) |

> **⚠️ NOTE:** Although map keys have to be serialized in *some* order, CBOR maps are considered *orderless*. This means that CBOR encoders will typically not treat the order of pairs as significant, and neither should you. Similarly, nothing in the CBOR specification requires that map keys be *unique*. Theoretically you could have multiple pairs with the same key, but many implementations will simply choose to keep one of the pairs and throw away the other. You should therefore never rely on the behavior of particular implementations regarding the order of keys or duplicate keys. Determinisic encoding profiles we'll discuss later in this book will address these ambiguities.

## Floating-Point and Other Simple Values (Major Type 7)

Major Type 7 serves as a catch-all for simple values (like `true`, `false`, and `null`, covered earlier) and floating-point numbers.

### Floating-Point Numbers (Major Type 7)

CBOR supports [IEEE-754](https://en.wikipedia.org/wiki/IEEE_754) binary floating-point numbers in half, single, and double precision. The Additional Information (AI) field specifies the precision, and the bytes that follow are in network byte order (big-endian).

| Precision           | AI Value | Bytes After Initial Byte  | Total Size | Notes                            |
|---------------------|----------|---------------------------|------------|----------------------------------|
| Half-precision      | 25       | 2 bytes                   | 3 bytes    | 16-bit float (`float16`)         |
| Single-precision    | 26       | 4 bytes                   | 5 bytes    | 32-bit float (`float32`)         |
| Double-precision    | 27       | 8 bytes                   | 9 bytes    | 64-bit float (`float64`)         |

#### Preferred Serialization for Floating-Point Numbers

Similar to integers, preferred serialization for floating point values dictates using the shortest floating-point representation that can exactly encode a given value. If a number can be precisely represented in `float16`, it is encoded that way instead of using `float32` or `float64`.

| Value        | CBOR Diagnostic  | CBOR Hex                     | MT | AI | Explanation                                                |
|--------------|------------------|------------------------------|----|----|------------------------------------------------------------|
| `0.0`        | `0.0`            | `f9 00 00`                   | 7  | 25 | Half-precision (`float16`); zero                           |
| `1.0`        | `1.0`            | `f9 3c 00`                   | 7  | 25 | Half-precision; `0x3c00` encodes `1.0`                     |
| `-1.5`       | `-1.5`           | `f9 be 00`                   | 7  | 25 | Half-precision; `0xbe00` encodes `-1.5`                    |
| `10000.0`    | `10000.0`        | `fa 47 c3 50 00`             | 7  | 26 | Single-precision; `0x47c35000` encodes `10000.0`           |
| `1.1`        | `1.1`            | `fb 3f f1 99 99 99 99 99 9a` | 7  | 27 | Double-precision; only this width exactly encodes `1.1`    |
| `3.14159`    | `3.14159`        | `fb 40 09 21 f9 f0 1b 86 6e` | 7  | 27 | Double-precision; needed to preserve exact π approximation |
| `1.0e+300`   | `1.0e+300`       | `fb 7e 37 e4 3c 88 00 75 9c` | 7  | 27 | Double-precision; high magnitude                           |
| `Infinity`   | `Infinity`       | `f9 7c 00`                   | 7  | 25 | Half-precision encoding for positive infinity              |
| `NaN`        | `NaN`            | `f9 7e 00`                   | 7  | 25 | Half-precision encoding for NaN (payload may vary)         |

### Other Simple Values

Besides `false`, `true`, `null`, and `undefined` (AI 20-23), Major Type 7 allows for simple values 0 through 19 (encoded directly with AI 0-19) and 32 through 255 (encoded using AI 24 followed by one byte). The specific meanings of these simple values are generally undefined by the core CBOR specification and are reserved for specific profiles or applications.

| Value Range | Encoding Method                            | Semantics              |
|-------------|--------------------------------------------|------------------------|
| 0–19        | MT 7, AI = value (1-byte)                  | Reserved               |
| 20          | MT 7, AI = 20 (0xf4)                       | `false`                |
| 21          | MT 7, AI = 21 (0xf5)                       | `true`                 |
| 22          | MT 7, AI = 22 (0xf6)                       | `null`                 |
| 23          | MT 7, AI = 23 (0xf7)                       | `undefined`            |
| 24          | MT 7, AI = 24, followed by 1 byte          | Reserved               |
| 25–27       | MT 7, AI = 25–27, followed by 2–8 bytes    | Floating-point numbers |
| 28–30       | MT 7, AI = 28–30                           | Reserved               |
| 31          | MT 7, AI = 31 (0xff)                       | "break" stop code      |
| 32–255      | MT 7, AI = 24, followed by 1 byte (value)  | Reserved               |

**Notes:**

- **Values 0–19** are currently unassigned and reserved for future use.
- **Values 20–23** represent the simple values `false`, `true`, `null`, and `undefined`, respectively.
- **Value 24** is reserved and not used for encoding simple values.
- **Values 25–27** are used to encode floating-point numbers of different precisions:
  - 25: Half-precision (16-bit)
  - 26: Single-precision (32-bit)
  - 27: Double-precision (64-bit)
- **Values 28–30** are reserved for future extensions.
- **Value 31** is used as a "break" stop code to indicate the end of an indefinite-length item.
- **Values 32–255** are unassigned and available for application-specific use.

For the most up-to-date information, refer to the [IANA CBOR Simple Values registry](https://www.iana.org/assignments/cbor-simple-values/cbor-simple-values.xhtml).

## Putting It Together: A Nested Example

Now let's combine these elements into a more complex, nested structure. Consider the following JSON object:

JSON

```
{
  "name": "Gadget",
  "id": 12345,
  "enabled": true,
  "parts": [
    "bolt",
    "nut"
  ],
  "spec": {
    "size": 10.5,
    "data": "AQAA/w=="
  }
}
```

Note that the `"data"` value in JSON is base64 encoded, representing the bytes `0x01, 0x00, 0x00, 0xff`. In CBOR, we can represent this directly as a byte string.

**CBOR Diagnostic Notation:**

```
{
  "name": "Gadget",
  "id": 12345,
  "enabled": true,
  "parts": [
    "bolt",
    "nut"
  ],
  "spec": {
    "size": 10.5,
    "data": h'010000ff'
  }
}
```

**CBOR Hexadecimal Encoding with Commentary:**

```
a5                     # map(5 pairs follow)
   64 6e616d65         # key 0: text (4 bytes, "name")
   66 476164676574     # value 0: text (6 bytes, "Gadget")
   62 6964             # key 1: text (2 bytes, "id")
   19 3039             # value 1: unsigned(12345)
   67 656e61626c6564   # key 2: text (7 bytes, "enabled")
   f5                  # value 2: primitive(21) (true)
   65 7061727473       # key 3: text(5 bytes, "parts")
   82                  # value 3: array(2 elements follow)
      64 626f6c74         # element 0: text(4 bytes, "bolt")
      63 6e7574           # element 1: text(3 bytes, "nut")
   64 73706563         # key 4: text(4 bytes, "spec")
   A2                  # value 4: map(2 pairs follow)
      64 73697A65         # key 0: text(4 bytes, "size")
      F9 4940             # value 0: float(10.5) (half-precision)
      64 64617461         # key 1: text(4 bytes, "data")
      44 010000FF         # value 1: bytes(4 bytes, h'010000FF')
```

This example demonstrates how the basic building blocks combine to represent complex, nested data structures efficiently.

## Conclusion: Foundations Laid

This chapter has laid the groundwork for understanding CBOR by dissecting its core encoding mechanism. We've seen how the header byte, through its Major Type and Additional Information fields, defines the structure and type of every data item. We explored the preferred binary representations for fundamental types inherited from the JSON data model – integers (positive and negative), booleans, null, text strings, arrays, and maps – along with CBOR's native byte strings and standard floating-point numbers. By consistently comparing JSON, CBOR Diagnostic Notation, and the raw hexadecimal CBOR, we've illuminated the direct mapping between the familiar data model and its concise binary encoding.

With this foundation, you should now be able to interpret the structure of basic CBOR data items encoded using definite lengths and preferred serialization. You understand how CBOR achieves compactness while remaining self-describing at a fundamental level, allowing decoders to process data without prior schema knowledge.

However, this is just the beginning of the CBOR story. We intentionally deferred several important features to establish this core understanding:

- **Semantic Tags (Major Type 6):** CBOR's powerful extensibility mechanism for adding meaning beyond the basic types.
- **Indefinite-Length Items:** Encoding strings, arrays, and maps when their final size isn't known upfront, crucial for streaming applications.
- **CBOR Sequences:** Transmitting multiple independent CBOR data items back-to-back in a stream.
- **Schema Definition (CDDL):** Formal languages like CDDL used to define and validate the structure of CBOR data.
- **Deterministic Encoding:** The stricter rules beyond preferred serialization needed to guarantee identical byte sequences for identical data, essential for cryptographic applications.

These advanced topics build upon the fundamentals covered here. In the upcoming chapters, we will explore CBOR's extensibility through tags, dive deep into the requirements and techniques for achieving deterministic encoding (dCBOR), and see how these elements combine to create robust, verifiable data structures like Gordian Envelope.
