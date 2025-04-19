# Indefinite-Length Items

## Introduction: Beyond Known Lengths

In [A Practical Introduction to CBOR](./practical_introduction_to_cbor.md), we established the foundational mechanics of CBOR encoding, focusing on how the initial byte(s) of a data item—through the interplay of Major Type (MT) and Additional Information (AI)—convey the item's type and, crucially, its size or value. We saw how integers, strings, arrays, and maps are typically encoded using _definite lengths_, where the exact number of bytes (for strings) or elements/pairs (for collections) is specified upfront using AI values 0 through 27. This approach, particularly when combined with preferred serialization rules, leads to compact and efficiently parsable representations, provided the size of the data item is known _before_ serialization begins.

However, there are common scenarios where determining the total size of a data item in advance is impractical, inefficient, or even impossible. Consider these situations:

- **Incremental Generation:** A system might generate a large log entry or document piece by piece, appending data as it becomes available. Calculating the final size would require buffering the entire content first.
- **Network Streaming:** Sensor data or results from a long-running computation might need to be transmitted over a network as soon as parts are ready, without waiting for the entire dataset to be complete.
- **Data Pipelines:** An intermediate process might receive data chunks from one source and need to forward them immediately in CBOR format to the next stage, without the memory or latency budget to assemble the complete object first.

For these kinds of streaming applications, requiring the total length upfront negates the benefits of incremental processing. CBOR addresses this challenge directly with _indefinite-length encoding_, a mechanism specifically designed for situations where the size of certain data items is not known when serialization starts. This alternative encoding applies only to byte strings (Major Type 2), text strings (Major Type 3), arrays (Major Type 4), and maps (Major Type 5).

This chapter delves into the practical details of indefinite-length CBOR encoding. We will explore the specific encoding mechanism, examine how strings and collections are represented using this method, discuss its primary use cases and practical implications for parsers, survey its application in real-world protocols, and crucially, understand why this flexible encoding is explicitly disallowed in deterministic CBOR profiles. By the end of this chapter, you will have a solid working knowledge of how CBOR handles streaming data and the trade-offs involved.

## The Indefinite-Length Mechanism: AI 31 and the "Break" Code

The core mechanism for indefinite-length encoding leverages a specific value within the Additional Information (AI) part of the initial byte, alongside a unique stop code.

Recall from the [Practical Introduction](./practical_introduction_to_cbor.md) chapter the structure of the initial byte in any CBOR data item:

```
┌────┬────┬────┬────┬────┬────┬────┬────┐
│  7 │  6 │  5 │  4 │  3 │  2 │  1 │  0 │
├────┴────┴────┼────┴────┴────┴────┴────┤
│  MAJOR TYPE  │ ADDITIONAL INFORMATION │
└──────────────┴────────────────────────┘
```

While AI values 0 through 27 are used to encode literal values or definite lengths/counts, the AI value 31 (binary `11111`) serves a distinct purpose related to indefinite-length items.

**Signaling the Start:** When AI value 31 is combined with the Major Types that support indefinite lengths (2, 3, 4, and 5), it signals the _start_ of an indefinite-length data item of that specific type. It essentially acts as a marker indicating, "An item of this type begins here, but its total length is not provided; subsequent data items or chunks will follow until a specific terminator is encountered."

**Applicable Major Types:** It is crucial to remember that indefinite-length encoding is _only_ defined for the following Major Types:

| Major Type | Description  |
|------------|--------------|
| 2          | Byte String  |
| 3          | Text String  |
| 4          | Array        |
| 5          | Map          |

Other Major Types (0, 1, 6, 7) do not have an indefinite-length encoding mechanism defined via AI 31 in this manner.

**The Universal Terminator: The `0xff` "Break" Code:** To signal the end of an indefinite-length sequence (whether it's chunks of a string or elements/pairs of a collection), CBOR defines a unique, single-byte stop code: `0xff`. This byte is often referred to as the "break" code.

The encoding of the break code itself is Major Type 7 (Simple Values / Floating-Point) with Additional Information 31. This specific combination (`111 11111` binary) is reserved solely for this purpose. Its structure ensures it cannot be mistaken for the start of any standard CBOR data item, making it an unambiguous terminator for indefinite-length sequences. A parser encountering `0xff` in a context where it's expecting the next chunk of an indefinite string, the next element of an indefinite array, or the next key/value of an indefinite map knows that the indefinite-length item is now complete.

The following table summarizes the specific initial bytes used to start indefinite-length items and the universal break code:

| Type                   | MT | AI | Encoding   | Description                       |
|------------------------|----|----|------------|-----------------------------------|
| Indefinite Byte String | 2  | 31 | `5f`       | Start of indefinite byte string   |
| Indefinite Text String | 3  | 31 | `7f`       | Start of indefinite text string   |
| Indefinite Array       | 4  | 31 | `9f`       | Start of indefinite array         |
| Indefinite Map         | 5  | 31 | `bf`       | Start of indefinite map           |
| Break Code             | 7  | 31 | `ff`       | End of any indefinite-length item |

Understanding these specific byte values (`5f`, `7f`, `9f`, `bf` for starting, `ff` for stopping) is key to recognizing and parsing indefinite-length CBOR data streams.

## Streaming Data: Indefinite-Length Strings

Indefinite-length strings provide a way to encode byte sequences or UTF-8 text without knowing the total number of bytes beforehand. They achieve this by breaking the string content into manageable chunks.

The fundamental concept is that an indefinite-length string is represented as:

1. The specific start marker (`5f` for byte strings, `7f` for text strings).
2. A sequence of zero or more _definite-length_ string chunks of the _same_ major type.
3. The `0xff` break code.

The logical value of the complete string is obtained by concatenating the _content_ (the raw bytes or UTF-8 text, excluding the definite-length headers) of these chunks in the order they appear.

### Indefinite-Length Byte Strings (Major Type 2, AI 31)

- **Encoding Structure:** An indefinite-length byte string starts with `5f`, followed by zero or more definite-length byte string chunks, and terminates with `ff`.

```
5f [chunk1][chunk2]... ff
```

- **Chunk Structure:** Each `[chunkN]` must be a complete, definite-length byte string data item (Major Type 2, AI 0-27). For example, `43 010203` represents a chunk containing the 3 bytes `0x01`, `0x02`, `0x03`. An empty chunk, encoded as `40`, is also valid and contributes nothing to the concatenated value.
- **Examples:**
    - An empty byte string encoded using indefinite length:
        - CBOR Diagnostic: `_ h''`
        - CBOR Hex: `5f ff`
    - The byte sequence `0x01, 0x02, 0x03, 0x04, 0x05` encoded indefinitely with two chunks:
        - CBOR Diagnostic: `_ h'010203' h'0405'`
        - CBOR Hex: `5f 43 010203 42 0405 ff`
            - `5f`: Start indefinite byte string
            - `43 010203`: Chunk 1 (definite length 3, bytes `01 02 03`)
            - `42 0405`: Chunk 2 (definite length 2, bytes `04 05`)
            - `ff`: Break code
    - The same byte sequence encoded indefinitely with a single chunk:
        - CBOR Diagnostic: `_ h'0102030405'`
        - CBOR Hex: `5f 45 0102030405 ff`
            - `5f`: Start indefinite byte string
            - `45 0102030405`: Chunk 1 (definite length 5, bytes `01 02 03 04 05`)
            - `ff`: Break code

Notice that the same logical byte sequence (`0102030405`) can be represented in multiple ways using indefinite-length encoding, depending on the chunking strategy. This flexibility is the core benefit for streaming, but it also introduces non-canonical representations. Furthermore, compared to the definite-length encoding (`45 0102030405`), the indefinite-length versions carry an overhead of at least two bytes (the `5f` start marker and the `ff` break code), plus the header bytes for each chunk. This trade-off between flexibility, overhead, and canonicality is central to understanding indefinite-length encoding.

### Indefinite-Length Text Strings (Major Type 3, AI 31)

- **Encoding Structure:** An indefinite-length text string starts with `7f`, followed by zero or more definite-length text string chunks, and terminates with `ff`.

```
7f [chunk1][chunk2]... ff
```

- **Chunk Structure:** Each `[chunkN]` must be a complete, definite-length _text_ string data item (Major Type 3, AI 0-27), meaning it must contain a sequence of bytes that constitutes valid UTF-8 encoding. For example, `63 666f6f` represents a chunk containing the 3 bytes for the UTF-8 string "foo".
- **UTF-8 Integrity Constraint:** This is a critical rule specific to indefinite-length _text_ strings: chunk boundaries **must not** occur in the middle of a multi-byte UTF-8 character sequence. Each individual chunk, when decoded, must result in a valid UTF-8 string. The concatenation of these valid chunks naturally forms the final, valid UTF-8 string. This constraint implies that an encoder generating indefinite-length text strings must be UTF-8 aware. When deciding where to split the text into chunks during streaming, it cannot simply cut after an arbitrary number of bytes; it must ensure the cut occurs only at a character boundary. This adds a layer of complexity compared to encoding indefinite-length byte strings, where chunks can be split arbitrarily.
- **Examples:**
    - An empty text string encoded using indefinite length:
        - CBOR Diagnostic: `_ ""`
        - CBOR Hex: `7f ff`
    - The text string `"Hello World"` encoded indefinitely with three chunks:
        - CBOR Diagnostic: `_ "Hello" " " "World"`
        - CBOR Hex: `7f 65 48656c6c6f 61 20 65 576f726c64 ff`
            - `7f`: Start indefinite text string
            - `65 48656c6c6f`: Chunk 1 ("Hello", definite length 5)
            - `61 20`: Chunk 2 (" ", definite length 1)
            - `65 576f726c64`: Chunk 3 ("World", definite length 5)
            - `ff`: Break code
    - The text string `"你好"` (UTF-8 bytes: `e4 bda0 e5 a5bd`) encoded indefinitely:
        - _Valid_ Chunking (split between characters):
            - CBOR Diagnostic: `_ "你" "好"`
            - CBOR Hex: `7f 63 e4bda0 63 e5a5bd ff` (Chunk 1: "你", length 3; Chunk 2: "好", length 3)
        - _Invalid_ Chunking (attempting to split within a character): An encoder must not produce, for example, a chunk ending in `e4 bd` followed by a chunk starting with `a0`. Each chunk's byte sequence must stand alone as valid UTF-8.

Similar to byte strings, indefinite-length text strings offer streaming flexibility at the cost of overhead and non-canonical representation, with the added requirement of maintaining UTF-8 validity within each chunk.

## Streaming Collections: Indefinite-Length Arrays and Maps

Just as strings can be streamed chunk by chunk, CBOR allows arrays and maps to be encoded without knowing the total number of elements or pairs upfront.

The principle is straightforward:

1. Start with the specific indefinite-length marker (`9f` for arrays, `bf` for maps).
2. Encode the elements (for arrays) or key-value pairs (for maps) sequentially, one after another.
3. Terminate the sequence with the `0xff` break code.

### Indefinite-Length Arrays (Major Type 4, AI 31)

- **Encoding Structure:** An indefinite-length array starts with `9f`, followed by zero or more encoded data items (its elements), and terminates with `ff`.

```
9f [item1][item2][item3]... ff
```

- **Element Structure:** Each `[itemN]` can be _any_ valid CBOR data item, including integers, strings (definite or indefinite), floats, booleans, null, tags, or even other arrays and maps (definite or indefinite).
- **Nesting:** Indefinite-length arrays can freely contain other indefinite-length items, allowing for complex, nested structures to be streamed.
- **Examples:**
    - An empty array encoded using indefinite length:
        - CBOR Diagnostic: `[_]`
        - CBOR Hex: `9f ff`
    - The array `[1, "two", true]` encoded indefinitely:
        - CBOR Diagnostic: `[_ 1, "two", true]`
        - CBOR Hex: `9f 01 63 74776f f5 ff`
            - `9f`: Start indefinite array
            - `01`: Element 1 (integer 1)
            - `63 74776f`: Element 2 (text string "two")
            - `f5`: Element 3 (true)
            - `ff`: Break code
    - A nested indefinite array `[_ "a", "b"]`:
        - CBOR Diagnostic: `[_ "a", "b"]`
        - CBOR Hex: `9f 01 9f 61 61 61 62 ff 03 ff`
            - `9f`: Start outer indefinite array
            - `01`: Outer element 1 (integer 1)
            - `9f`: Start inner indefinite array (Outer element 2)
            - `61 61`: Inner element 1 ("a")
            - `61 62`: Inner element 2 ("b")
            - `ff`: Break code for inner array
            - `03`: Outer element 3 (integer 3)
            - `ff`: Break code for outer array

### Indefinite-Length Maps (Major Type 5, AI 31)

- **Encoding Structure:** An indefinite-length map starts with `bf`, followed by zero or more key-value pairs encoded sequentially (key1, value1, key2, value2,...), and terminates with `ff`.

```
bf [key1][value1][key2][value2]... ff
```

- **Pair Structure:** Each key and each value can be _any_ valid CBOR data item. Crucially, the data items between the `bf` marker and the `ff` break code must come in pairs. A map must contain an even number of data items following the initial `bf`.
- **Nesting:** Indefinite-length maps can contain indefinite-length items as either keys or values.
- **Examples:**
    - An empty map encoded using indefinite length:
        - CBOR Diagnostic: `_ {}`
        - CBOR Hex: `bf ff`
    - The map `{"a": 1, "b": false}` encoded indefinitely:
        - CBOR Diagnostic: `_ {"a": 1, "b": false}`
        - CBOR Hex: `bf 61 61 01 61 62 f4 ff`
            - `bf`: Start indefinite map
            - `61 61`: Key 1 ("a")
            - `01`: Value 1 (integer 1)
            - `61 62`: Key 2 ("b")
            - `f4`: Value 2 (false)
            - `ff`: Break code
    - A map containing an indefinite-length byte string as a value `{"data": _ h'01' h'02'}`:
        - CBOR Diagnostic: `_ {"data": _ h'01' h'02'}`
        - CBOR Hex: `bf 64 64617461 5f 41 01 41 02 ff ff`
            - `bf`: Start indefinite map
            - `64 64617461`: Key ("data")
            - `5f`: Start indefinite byte string (Value)
            - `41 01`: Chunk 1 (`h'01'`)
            - `41 02`: Chunk 2 (`h'02'`)
            - `ff`: Break code for byte string
            - `ff`: Break code for map

The requirement for an even number of items between `bf` and `ff` is an important validation check for parsers. If a parser encounters the `ff` break code immediately after reading a key but before reading its corresponding value, it indicates a malformed indefinite-length map. This adds a slight amount of state tracking (ensuring pairs are complete) compared to parsing indefinite-length arrays.

## Use Cases and Practical Considerations

The primary motivation for indefinite-length encoding is to support streaming scenarios where data sizes are unknown upfront.

- **Network Protocols:** In protocols designed for constrained environments or transferring large objects, the ability to send data in chunks without pre-calculating the total size is valuable. CoAP (Constrained Application Protocol) Block-Wise Transfers (([RFC-7959](https://datatracker.ietf.org/doc/html/rfc7959))) is often cited in this context. While CoAP itself manages the blocking at the protocol level, and the payloads within those blocks are often CBOR using definite lengths for simplicity, the overall concept aligns with handling large data incrementally. Indefinite-length CBOR _could_ be used within such frameworks, although definite-length chunks are common in practice.
- **Log Generation/Aggregation:** Systems that generate extensive logs or aggregate log streams from various sources can benefit. An application can start an indefinite-length array or map for a log record, append fields (potentially including large, streamed strings) as they become available, and finalize the record with the break code without needing to buffer the entire structure in memory first.
- **Data Pipelines:** When CBOR data flows through multiple processing stages, using indefinite-length encoding can sometimes avoid the need for intermediate stages to buffer entire large strings or collections just to determine their length before passing them on.

However, using indefinite-length items introduces practical considerations for implementation:

- **Parser Implementation:** Parsing definite-length items is often simpler. The parser reads the length L, potentially allocates memory for L bytes or L items, and then reads exactly that amount of data. Parsing indefinite-length items requires a different logic: the parser reads the start marker (`5f`/`7f`/`9f`/`bf`), then enters a loop, reading one complete data item (a chunk, an element, or a key-value pair) at a time. After each item, it must check if the next byte is the `0xff` break code. If not, it continues the loop; if it is, the indefinite item is complete. This typically involves more state management within the parser.
- **Buffering Considerations:** While indefinite-length encoding allows the _sender_ to stream data without knowing the total size, it doesn't automatically eliminate the need for buffering on the _receiver's_ side. If the receiving application needs the entire concatenated string value, or needs access to all array elements simultaneously, before it can perform its processing, it will still have to accumulate the incoming chunks or elements in memory until the `0xff` break code is received. The primary benefit of streaming often accrues to the sender by reducing memory requirements and latency-to-first-byte, but the receiver's processing model dictates whether it can also process the data incrementally or must buffer.
- **Nesting Complexity:** Parsing nested indefinite-length items requires careful management. When a parser encounters an indefinite-length start marker while already parsing another indefinite-length item, it must correctly associate the eventual `0xff` break codes with their corresponding start markers. This is typically handled using a stack internally within the parser to keep track of the nesting depth and the type of indefinite item currently being parsed.

## Indefinite-Length Items in the Wild

While indefinite-length encoding is a standard part of the CBOR specification (([RFC-8949](https://datatracker.ietf.org/doc/html/rfc8949))), its adoption in specific protocols and applications appears less widespread than definite-length encoding.

As mentioned, CoAP Block-Wise Transfers (([RFC-7959](https://datatracker.ietf.org/doc/html/rfc7959))) provides a mechanism conceptually similar to streaming, allowing large resources (which might be represented in CBOR) to be transferred in chunks over constrained networks. However, the specification focuses on the CoAP-level blocking and doesn't mandate the use of CBOR indefinite-length encoding _within_ those blocks. Implementations often favor definite-length CBOR for the block payloads due to simpler handling and the deterministic nature often desired, even if the overall resource size isn't known initially by the CoAP endpoints.

Finding other prominent, standardized protocols that _mandate_ or heavily rely on CBOR indefinite-length encoding can be challenging. This might be partly attributed to the implications for deterministic encoding (discussed next) and the fact that many applications prioritize predictability or can manage buffering to determine definite lengths.

Nonetheless, the mechanism exists as a standard tool for scenarios where a sender truly cannot determine the size beforehand, particularly in highly resource-constrained environments or pure streaming pipelines where avoiding buffering on the sender side is paramount.

## Why Not Deterministic? The Canonical Conundrum

One of the most significant implications of indefinite-length encoding is its incompatibility with _deterministic_ encoding requirements.

**The Goal of Deterministic Encoding:** As outlined in RFC 8949, Section 4.2 ("Core Deterministic Encoding Requirements"), and forming the basis for profiles like dCBOR, the primary goal of deterministic encoding is to ensure that any given CBOR data model instance has **exactly one**, unambiguous, canonical binary representation. This property is absolutely critical for several use cases:

- **Cryptographic Signatures:** To verify a digital signature over CBOR data, the verifier must be able to reconstruct the exact sequence of bytes that was originally hashed and signed. If multiple valid encodings exist for the same logical data, signature verification becomes unreliable or impossible.
- **Hashing:** When using cryptographic hashes for data integrity checks, content addressing (like in distributed systems or blockchains), or indexing, it's essential that identical data always produces the identical hash. This requires a single, canonical byte representation.
- **Data Comparison:** In databases or distributed systems, comparing data items for equality often relies on simple byte-wise comparison for efficiency. This only works correctly if the encoding is canonical.

**The Ambiguity of Indefinite-Length:** Indefinite-length encoding fundamentally breaks the canonical requirement because it allows the same logical data (a specific string, array, or map) to be encoded into multiple, different byte sequences based solely on how the sender chooses to chunk the data (for strings) or simply by virtue of using the indefinite markers instead of definite ones.

Consider the simple byte string `h'01020304'`:

- **Definite-Length Encoding (Canonical):** `44 01020304` (1 initial byte + 4 content bytes = 5 bytes total)
- **Indefinite-Length (1 chunk):** `5f 44 01020304 ff` (1 start byte + 1 chunk header byte + 4 content bytes + 1 break byte = 7 bytes total)
- **Indefinite-Length (2 chunks):** `5f 42 0102 42 0304 ff` (1 start + 1+2 chunk1 + 1+2 chunk2 + 1 break = 8 bytes total)
- **Indefinite-Length (4 chunks):** `5f 41 01 41 02 41 03 41 04 ff` (1 start + 4*(1+1) chunks + 1 break = 10 bytes total)

All four representations above correspond to the same logical sequence of four bytes. However, they result in distinct binary encodings (`44...`, `5f 44...`, `5f 42...`, `5f 41...`).

**Violation of Canonical Requirement:** This inherent possibility of multiple valid byte sequences for identical data directly violates the core principle of deterministic, canonical encoding. There is no single "preferred" way to chunk an indefinite-length string, making the representation inherently ambiguous from a byte-sequence perspective.

**Exclusion from Deterministic Profiles:** Consequently, specifications defining deterministic CBOR encoding, such as RFC 8949 Section 4.2.2 ("Length-Determinism"), explicitly **forbid** the use of indefinite-length encoding. Any data item whose initial byte is `5f`, `7f`, `9f`, or `bf` is disallowed in contexts requiring Core Deterministic Encoding or similar canonical profiles. This exclusion is not arbitrary; it is a necessary consequence of prioritizing byte-for-byte reproducibility over the flexibility offered by indefinite-length streaming. Applications requiring canonical forms _must_ use definite-length encoding, which necessitates knowing the size of strings and the counts for collections before serialization.

## Conclusion: Flexibility vs. Predictability

Indefinite-length encoding stands as a specialized feature within the CBOR standard, designed to address the practical challenge of serializing data whose size is unknown when encoding begins. By using dedicated start markers (`5f`, `7f`, `9f`, `bf`) based on Major Type combined with Additional Information 31, and a universal `0xff` break code, CBOR allows byte strings, text strings, arrays, and maps to be constructed incrementally. For strings, this involves concatenating definite-length chunks; for collections, it involves appending elements or key-value pairs sequentially until the break code is encountered.

The primary advantage of this mechanism is its ability to support streaming applications, enabling senders (especially those with limited memory or needing low latency) to transmit data without first buffering the entire object to calculate its size.

However, this flexibility comes with significant trade-offs, including non-canonical representations leading to exclusion from deterministic profiles, potential overhead in encoding size, and increased complexity in parsing logic. The requirement for UTF-8 integrity in indefinite-length text strings and dealing with nested indefinite items adds further complexity for implementers.
