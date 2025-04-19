# Introducing CBOR


### Designing CBOR: Binary Encoding, Lengths, and Tags

CBOR achieves its compactness and efficiency through a carefully designed binary encoding scheme. Every piece of CBOR data, called a **data item**, begins with a single **header byte**. This byte contains two crucial pieces of information:

- **Major Type (MT)**: The first 3 bits indicate the general type of data item.
- **Additional Information**: The remaining 5 bits provide information specific to the major type, often a small value or count, or an indicator that more bytes follow for larger values/lengths.

There are 8 major types (MT 0-7) 103:

- **MT 0**: Unsigned Integer (value encoded directly or in following bytes)
- **MT 1**: Negative Integer (value encoded directly or in following bytes)
- **MT 2**: Byte String (length encoded, followed by raw bytes)
- **MT 3**: Text String (length encoded, followed by UTF-8 encoded bytes)
- **MT 4**: Array (number of elements encoded, followed by the elements)
- **MT 5**: Map (number of key-value _pairs_ encoded, followed by alternating keys and values)
- **MT 6**: Semantic Tag (tag number encoded, followed by the tagged data item)
- **MT 7**: Simple Values and Floating-Point Numbers (e.g., `false`, `true`, `null`, `undefined`, floats)

The encoding of the value or length depends on the 5-bit additional information field:

- **Values 0-23**: Represent the actual unsigned integer value (for MT 0, 1, 7) or the length in bytes/items (for MT 2-5) directly.
- **Value 24**: Indicates the value/length is contained in the _next 1 byte_ (uint8_t).
- **Value 25**: Indicates the value/length is contained in the _next 2 bytes_ (uint16_t).
- **Value 26**: Indicates the value/length is contained in the _next 4 bytes_ (uint32_t).
- **Value 27**: Indicates the value/length is contained in the _next 8 bytes_ (uint64_t).
- **Values 28-30**: Reserved for future extensions.
- **Value 31**: Special meaning: indicates **indefinite length** for strings, arrays, and maps (MT 2-5), or the **break stop code** for MT 7.

This structure allows for very compact encoding of small integers and short strings/arrays/maps, while scaling efficiently to handle large values and lengths up to 264−1. All multi-byte integers (lengths, values, tag numbers) are encoded in network byte order (big-endian).

A key feature of CBOR is its support for **indefinite lengths** for byte strings, text strings, arrays, and maps. When the additional information field is 31 (0x1F), it signals that the size is not known beforehand. For strings (MT 2, 3), the content is encoded as a sequence of zero or more definite-length chunks of the _same_ major type, terminated by a special **break** or **stop code** data item (`0xFF`, which is MT 7, additional info 31). For arrays (MT 4) and maps (MT 5), the elements or key-value pairs are simply encoded one after another until the break code `0xFF` appears. This mechanism is particularly useful for streaming applications where the total size of the data may not be available when encoding begins. However, it requires decoders to handle potentially unbounded input until the break code is encountered, which has drawn some criticism regarding potential memory allocation issues if not implemented carefully. Implementations can use techniques like pre-allocating buffers or using "fixups" in constrained environments to manage this.

**Semantic Tags (MT 6)** provide CBOR's extensibility mechanism. A tag consists of the MT 6 header (with the tag number encoded using the same rules as integers) followed by a single data item (the content being tagged). These tags add semantic meaning beyond the basic CBOR data model. For example, tag 1 indicates that the following unsigned integer (MT 0) or float (MT 7) represents an epoch-based date/time. Tag 2 indicates a positive bignum, tagged onto a byte string (MT 2). Tag 32 indicates a URI, tagged onto a text string (MT 3). Tag 34 signals that a text string should be interpreted as Base64url encoded data. Tag numbers are managed through an IANA registry, allowing for both standardized and application-specific extensions. A key aspect is that decoders are not required to understand all tags; they can process the underlying data item even if the tag itself is unrecognized, allowing for graceful evolution.
