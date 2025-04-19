# Extending Semantics with CBOR Tags

## Beyond Basic Types: The Need for Meaning

In the previous chapter, we explored the fundamental mechanics of CBOR encoding, focusing on how basic data types like integers, strings, arrays, and maps are represented in a compact binary form. We saw how CBOR leverages a simple initial byte structure (Major Type and Additional Information) to create a self-describing format at the byte level, closely mirroring the familiar JSON data model but optimized for efficiency.

However, real-world data often carries meaning beyond these fundamental structures. How do we distinguish a simple integer representing a count from one representing seconds since an epoch? How do we represent a date, a URI, or a number larger than standard 64-bit integers can hold? While applications could implicitly agree on the meaning of specific fields (e.g., "the 'timestamp' field is always epoch seconds"), this approach lacks standardization and can lead to ambiguity and interoperability issues.

CBOR addresses this need for richer semantics through its **tagging mechanism**. Tags allow us to annotate underlying data items, providing additional context or type information without fundamentally changing the encoding structure. They are a cornerstone of CBOR's extensibility, enabling the representation of a vast range of data types beyond the core set, from standard types like dates and URIs to application-specific structures.

This chapter delves into CBOR Tags (Major Type 6). We will explore:

- How tags work mechanically.
- Their purpose in adding semantic meaning and enabling extensibility.
- The IANA registry that standardizes tag definitions.
- The different ranges of tag numbers and their implications for interoperability.
- A selection of commonly used ("notable") tags with practical examples.

By the end of this chapter, you will understand how to use and interpret CBOR tags, unlocking a powerful feature for representing complex and meaningful data structures efficiently.

> **⚠️ NOTE:** As before, this chapter aims for practical understanding. For definitive details, always refer to the official specification,([https://datatracker.ietf.org/doc/html/rfc8949](https://datatracker.ietf.org/doc/html/rfc8949)), and the IANA registries it defines.

## Tagging Mechanism (Major Type 6)

CBOR dedicates Major Type 6 specifically for tags. A tag consists of two parts:

1. **Tag Number:** An unsigned integer (ranging from 0 up to 2⁶⁴−1) that identifies the tag's meaning.
2. **Tag Content:** A single, subsequent CBOR data item that is being tagged.

```
┌──────────────────────┐
│   TAG HEADER BYTE    │   → Major Type 6 + AI (determines length of tag number)
├──────────────────────┤
│   TAG NUMBER BYTES   │   → (0 to 8 bytes depending on AI)
└──────────────────────┘
           ↓
┌──────────────────────┐
│   TAGGED DATA ITEM   │   → Any valid CBOR item (primitive, array, map, etc.)
└──────────────────────┘
```

The encoding follows the standard CBOR pattern. The initial byte has its high-order 3 bits set to `110` (Major Type 6). The low-order 5 bits (Additional Information) encode the tag number itself, using the same rules used for all the major types:

| Tag Number Range    | Initial Byte     | Additional Bytes | Total Tag Header Size | Notes                   |
|---------------------|------------------|------------------|-----------------------|-------------------------|
| 0 to 23             | `0xC0` to `0xD7` | None             | 1 byte                | Tag number in AI (0–23) |
| 24 to 255           | `0xD8`           | 1 byte (uint8)   | 2 bytes               | AI = 24                 |
| 256 to 65535        | `0xD9`           | 2 bytes (uint16) | 3 bytes               | AI = 25                 |
| 65536 to 4294967295 | `0xDA`           | 4 bytes (uint32) | 5 bytes               | AI = 26                 |
| 4294967296 to 2⁶⁴−1 | `0xDB`           | 8 bytes (uint64) | 9 bytes               | AI = 27                 |

Immediately following the initial byte(s) that encode the tag number comes the complete encoding of the single data item that serves as the tag's content.

**Example:** Tag 2 (unsigned bignum) applied to the byte string `h'0102'`

| CBOR Hex | MT | AI | Explanation                                      |
|----------|----|----|--------------------------------------------------|
| `c2`     | 6  | 2  | Tag(2): Major Type 6, AI encodes tag number 2    |
| `42`     | 2  | 2  | Byte String (Major Type 2), length = 2 bytes     |
| `0102`   | –  | –  | Tag Content: raw bytes `0x01`, `0x02`            |

**CBOR Diagnostic Notation:** `2(h'0102')`

> **⚠️ NOTE:** If you put this diagnostic notation into the [CBOR playground](https://cbor.me/), convert it to its hexadecimal representation and back, you will get the value `258`! This is because the playground understands that byte strings tagged with **Tag 2** (unsigned bignum) are interpreted as a single integer value. In this case, the first byte `0x01` is the most significant byte, and the second byte `0x02` is the least significant byte, leading to the calculation: `(1 * 256 + 2) = 258`. This is the playground enforcing *preferred serialization* of numbers, which is a feature of the playground, not a requirement of CBOR itself.

### Purpose of Tags

Why introduce this extra layer? Tags serve several crucial purposes aligned with CBOR's design goals:

- **Adding Semantics:** Tags provide standardized meaning to underlying data. Tag 1 indicates that an integer or float represents epoch-based time; Tag 32 indicates a text string is a URI. This allows applications to interpret data correctly without relying solely on field names or out-of-band agreements.
- **Extensibility:** Tags are CBOR's primary mechanism for defining new data types beyond the basic set, without requiring version negotiation. New standards or applications can define tags for specialized data structures (like cryptographic keys, geographic coordinates, or domain-specific objects) and register them, allowing the CBOR ecosystem to grow organically.
- **Interoperability Hints:** Some tags provide guidance for converting CBOR data to other formats, particularly JSON which lacks native support for types like byte strings or dates. Tags 21-23, for example, suggest how binary data might be represented using base64 or hex encoding if conversion is necessary.
- **Type System Augmentation:** Tags allow CBOR to represent data types common in programming languages but not directly present in the basic JSON model, such as unsigned 64-bit integers, arbitrarily large integers (bignums), specific date/time formats, UUIDs, and more.

This mechanism of using an inline prefix tag number followed by the content provides a compact, binary-native way to convey type and semantic information. This contrasts with more verbose text-based approaches like XML namespaces or JSON-LD contexts, aligning with CBOR's goal of message size efficiency.

### Decoder Behavior

Crucially, CBOR decoders are **not required** to understand the semantics of every tag they encounter. This is a key aspect of CBOR's extensibility and forward compatibility. A generic decoder encountering an unknown tag `N` followed by content `C` can simply:

1. Decode the tag number `N`.
2. Decode the tag content `C`.
3. Pass both `N` and `C` to the application.

The application can then decide whether it understands tag `N` and how to interpret `C` based on it. If the application doesn't recognize tag `N`, it might treat `C` as opaque data, ignore it, or raise an error, depending on the application's logic. This allows systems to process messages containing newer, unknown tags without failing, provided the application logic can handle the tagged data appropriately (perhaps by ignoring it).

### Tag Nesting

Tags can be nested. A tag can enclose another tag, which in turn encloses a data item. For example, consider `TagA(TagB(ItemC))`. The interpretation applies from the inside out: `TagB` modifies or adds semantics to `ItemC`, and then `TagA` applies to the result of `TagB(ItemC)`.

Later in the book we'll discuss Gordian Envelope. The CBOR diagnostic notation for a very simple envelope containing just a text string might look like this:

```
200(201("Hello, envelope!"))
```

Tag 200 is registered with IANA as "Gordian Envelope". So anytime you encounter tag 200, you know you're looking at a Gordian Envelope. The tag 201 represents dCBOR (deterministic CBOR), which we'll also cover in this book. In the Gordian Envelope specification, an Envelope containing just dCBOR is a `LEAF` node, which can be any valid dCBOR-- in this case, a text string.

## Finding Your Tags: The IANA Registry

With potentially 2⁶⁴ tag numbers available, how do we ensure that different applications don't use the same number for conflicting purposes? With that many tags, you could just pick them at random and be pretty certain nobody else is using them, but there's a better way! The Internet Assigned Numbers Authority (IANA) maintains the official [Concise Binary Object Representation (CBOR) Tags registry](https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml).

This registry serves as the central, authoritative source for standardized tag assignments. Its importance cannot be overstated:

- **Interoperability:** The registry ensures that a specific tag number (especially in the lower, standardized ranges) consistently refers to the same semantic meaning and expected data item type across different implementations and protocols that adhere to the standards. This prevents conflicts where one application might use tag `X` for dates while another uses it for URIs.
- **Discovery:** It provides a public catalog where developers can look up existing tags for common data types (like dates, bignums, UUIDs, MIME messages, etc.) before defining their own. This encourages reuse and avoids unnecessary proliferation of tags for the same concept.

The registry is presented as a table with columns including:

- **Tag:** The tag number.
- **Data Item:** The expected CBOR type(s) of the tag content (e.g., text string, byte string, array, integer).
- **Semantics:** A brief description of the tag's meaning.
- **Reference:** A pointer to the document (an RFC or other stable specification) that defines the tag in detail.

## Tag Number Ranges and Registration Procedures

The IANA registry doesn't treat all tag numbers equally. The vast space from 0 to 264−1 is divided into distinct ranges, each with its own allocation policy. These policies reflect the intended use and required level of standardization for tags within that range. Understanding these ranges is crucial for choosing appropriate tags and understanding their interoperability implications.

The primary ranges and their procedures are:

1. **Range 0-23 (Standards Action)**

    - **Encoding:** These are the most compact tags, encoded directly within the initial byte (`0xc0` to `0xd7`).
    - **Procedure:** Requires **Standards Action**. Assignment typically necessitates definition within an IETF Request for Comments (RFC) or a standard from another recognized body. This is the most rigorous process, requiring that the IETF adopt the tag as part of a formal standard.
    - **Intended Use:** Reserved for core, fundamental, and widely applicable data types expected to be broadly interoperable (e.g., standard date/time formats, bignums, basic content hints).

2. **Range 24-32767 (Specification Required)**

    - **Encoding:** Covers tags requiring 1 additional byte (`0xd8 xx`, for tags 24-255) and the lower half of tags requiring 2 additional bytes (`0xd9 xxxx`, for tags 256-32767).
    - **Procedure:** Requires **Specification Required**. This means a stable, publicly accessible specification document defining the tag's semantics, expected data item format, and intended use must exist. IANA-appointed experts review the specification before registration. It's less formal than full Standards Action but still requires clear documentation and review.
    - **Intended Use:** Suitable for well-defined data types used within specific protocols, communities, or domains (e.g., COSE security tags, MIME messages, UUIDs, URIs, dCBOR, and Gordian Envelope). These tags are expected to be interoperable among parties using the defining specifications.

3. **Range 32768 - 18446744073709551615 (First Come First Served - FCFS)**

    - **Encoding:** Covers the upper half of 2-byte tags, all 4-byte tags (`0xda xxxxxxxx`), and all 8-byte tags (`0xdb xxxxxxxxxxxxxxxx`). This is the vast majority of the tag number space.
    - **Procedure:** **First Come First Served (FCFS)**. Registration is granted to the first applicant who provides the required information (based on the RFC 8949 template), including contact details and preferably a URL pointing to a description of the semantics. The review is primarily for completeness, not semantic detail or overlap (beyond the number itself).
    - **Intended Use:** Designed for application-specific tags, experimental use, vendor-specific extensions, or types where broad standardization isn't necessary or desired. Useful for rapid development or closed ecosystems.

This tiered structure represents a deliberate design choice, reflecting a spectrum from highly standardized and stable core types to flexible application-specific extensions. It reserves the most efficiently encoded tag numbers (0-23) for the most common, universally understood types, while providing ample space for innovation and specific needs in the higher ranges. All registration requests, regardless of range, must follow the basic template defined in RFC 8949 to ensure a minimum level of documentation.

## Choosing and Using Tags Wisely

The existence of different registration ranges has direct practical consequences for developers choosing tags:

- **Interoperability Guarantees:**

    - **Standards Action (0-23):** Offers the highest likelihood of interoperability. Implementations aiming for broad CBOR compliance should recognize and potentially handle these tags. Use them whenever your data semantically matches a tag in this range.
    - **Specification Required (24-32767):** Provides good interoperability _within the community that uses the defining specification_. Consumers outside this community may not recognize the tag without consulting the specification. Ideal for domain-specific standards (e.g., security tokens, IoT protocols).
    - **FCFS (32768+):** Offers the lowest _inherent_ interoperability guarantee. Use primarily for private or application-specific data types where producers and consumers are tightly coupled or have explicitly agreed on the tag's meaning. Relying on FCFS tags for broad, unspecified interoperability is risky.

> **⚠️ NOTE:** **The Danger of "Squatting"** Never use an _unregistered_ tag number from the Standards Action (0-23) or Specification Required (24-32767) ranges for your own private or experimental purposes. This practice, sometimes called "tag squatting," inevitably leads to collisions when IANA officially assigns that number for a different purpose. It breaks interoperability and creates significant problems down the line. Use the FCFS range for experimentation or application-specific needs.

## How to Register your own FCFS Tags

> **✅ TIP:** There is no charge to register a new tag.

1. Check the [IANA CBOR Tags Registry](https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml) to ensure that there isn't already an existing tag that does what you want. If you find one, use that instead of creating a new one.
2. Write your specification. This should be a stable, publicly accessible document that defines the tag's semantics, expected data item format, and intended use. It can be as simple as a [GitHub Gist](https://gist.github.com/), but it should be clear, unambiguous, and have a stable URL.
3. Check the [IANA CBOR Tags Registry](https://www.iana.org/assignments/cbor-tags/cbor-tags.xhtml) to ensure the tag you want isn't already taken.
4. Review [the section of RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html#name-cbor-tags-registry) that describes the registry, the registration process for tags, and the template for submitting a registration request.
5. Review the [IANA list of protocol registries](https://www.iana.org/protocols). You'll find the one called `CBOR Tags`, which also lists the IANA experts assigned to review tag registrations if they are in the *Specification Required* range.
6. Fill out the [IANA "General Request for Assignments" form](https://www.iana.org/form/protocol-assignment).

The form itself is very simple. You will need to provide:

- Your name and email address.
- The type of assignment you're requesting (CBOR Tags).
- The registry you're requesting the assignment from (the CBOR Tags registry).
- A reason for the assignment. This information is optional, but helpful and recommended.
- "Additional Information". For each tag, you're registering provide information corresponding to a column in the IANA registry. We recommend you review at the registry for examples:
    - **Tag:** The tag number.
    - **Data Item:** The expected CBOR type(s) of the tag content (e.g., text string, byte string, array, integer).
    - **Semantics:** A brief description of the tag's meaning.
    - **Reference:** A pointer to the document (an RFC or other stable specification) that defines the tag in detail.

That's it! Submit the form, and IANA will respond to your request by email.

## Notable Tags

The IANA CBOR Tags registry is authoritative and growing, listing hundreds of registered tags. Navigating this full list can be daunting. Fortunately, the IETF community maintains a document, [Notable CBOR Tags](https://datatracker.ietf.org/doc/draft-bormann-cbor-notable-tags/), which serves as a curated guide or "roadmap" to a selection of the most commonly used, interesting, or otherwise "notable" tags, particularly those defined since the original CBOR specification.

The [Internet Draft on Notable Tags](https://datatracker.ietf.org/doc/draft-bormann-cbor-notable-tags/) provides a number of tags in other interesting categories, including:

- **RFC 7049 (original CBOR specification)**
    Tags defined in the original CBOR specification, including standard date/time strings, bignums, decimal fractions, and base64 encodings.

- **Security**
    Tags used in security contexts, such as COSE (CBOR Object Signing and Encryption) and CBOR Web Tokens (CWT).

- **CBOR-based Representation Formats**
    Tags used in CBOR-based representation formats like YANG-CBOR.

- **Protocols**
    Tags utilized in specific protocols, including DOTS (DDoS Open Threat Signaling) and RAINS (Another Internet Naming Service).

- **Datatypes**
    Tags representing advanced datatypes, such as advanced arithmetic types, variants of undefined, and typed/homogeneous arrays.

- **Domain-Specific**
    Tags tailored for specific domains, including human-readable text and extended time formats.

- **Platform-Oriented**
    Tags related to specific platforms or programming languages, such as Perl, JSON, and unusual text encodings.

- **Application-Specific**
    Tags designed for particular applications, including enumerated alternative data items.

- **Implementation Aids**
    Tags intended to assist with implementation, such as invalid tags and programming aids for simple values.

> **✅ TIP:** While the IANA registry is the definitive source, the "Notable CBOR Tags" draft provides valuable context and summaries for many practical tags.

## A Few Commonly Used Tags

Let's explore a few of the most fundamental and useful tags, many defined in the original CBOR specification and detailed further in the notable tags draft:

### Tag 0: Standard Date/Time String

- **Content:** UTF-8 string
- **Semantics:** Represents a date and time expressed as a string, following the standard format defined in [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) (a profile of ISO 8601). This is a human-readable format.
- **Diagnostic:** `0("2013-03-21T20:04:00Z")`
- **Hex Example:**

```
C0                                      # tag(0)
   74                                   # text(20)
      323031332D30332D32315432303A30343A30305A # "2013-03-21T20:04:00Z"
```

### Tag 1: Epoch-Based Date/Time

- **Content:** Integer or Floating-point number
- **Semantics:** Represents a point in time as a numeric offset (in seconds, with optional fractional part for floats) from the standard Unix epoch (1970-01-01T00:00:00Z UTC). More compact and suitable for computation than Tag 0.
- **Diagnostic (Integer):** `1(1363896240)`
- **Hex Example (Integer):**

```
C1             # tag(1)
   1A 514B67B0 # unsigned(1363896240)
```

- **Diagnostic (Float):** `1(1698417015.123)`
- **Hex Example (Float - double precision):**

```
C1                     # tag(1)
   FB 41D94EF25DC7DF3B # 1698417015.123
```

> **✅ TIP:** The choice between integer and float depends on the need for sub-second precision. More advanced time tags exist (e.g., Tag 1001) offering higher precision and timescale information, but Tag 1 remains the basic epoch representation.

### Tag 2 and 3: Bignums

- **Content:** Byte string
- **Semantics:** Represents an arbitrarily large non-negative integer (Tag 2) or negative integer (Tag 3) that does not have to fit into the 64-bit unsigned integer (Major Type 0). The byte string contains the magnitude of the integer in network byte order (big-endian), with no leading zero bytes permitted in preferred/deterministic encoding.
- **Diagnostic (representing 18446744073709551616):** `2(h'010000000000000000')`
- **Hex Example (representing 18446744073709551616):**

```
C2                         # Tag(2, non-negative bignum)
    49 010000000000000000  # Byte String (length 9 bytes, 18446744073709551616)
```

### Tag 32: URI

- **Content:** UTF-8 string
- **Semantics:** Identifies the text string content as a Uniform Resource Identifier according to([RFC-3986](https://datatracker.ietf.org/doc/html/rfc3986)).
- **Diagnostic:** `32("http://cbor.io/")`
- **Hex Example:**

```
D8 20                                # tag(32)
   6F                                # text(15)
      687474703A2F2F63626F722E696F2F # "http://cbor.io/"
```

**Tag 37: UUID**

- **Content:** Byte string (must be 16 bytes long)
- **Semantics:** Identifies the byte string content as a Universally Unique Identifier, as defined in([RFC-9562](https://datatracker.ietf.org/doc/html/rfc9562)).
- **Diagnostic:** `37(h'f81d4fae7dec11d0a76500a0c91e6bf6')`
- **Hex Example:**

```
D8 25                   # Tag(37) - uses 1+1 encoding (0xd8 0x25)
    50                   # Byte String (length 16 bytes)
        f81d4fae7dec11d0a76500a0c91e6bf6
```

## Example: Tags in Action:

Let's see how these tags combine with basic CBOR types to represent a more complex data structure. Consider this JSON object representing a hypothetical sensor reading message:

**JSON:**

```
{
  "sensorID": "urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "captureTime": "2023-10-27T14:30:15.123Z",
  "reading": -12.345,
  "readingScale": -3,
  "rawValue": -12345,
  "statusURL": "https://example.com/status/f81d4fae",
  "alertPayload": "AQIDBA=="
}
```

Here, `sensorID` is a UUID, `captureTime` is a standard timestamp, `reading` could be represented as a decimal fraction (`-12345 * 10^-3`), `statusURL` is a URI, and `alertPayload` is base64-encoded binary data (`0x01020304`).

**CBOR Diagnostic Notation (using tags):**

```
{
  "sensorID": 37(h'f81d4fae7dec11d0a76500a0c91e6bf6'),    // Tag 37 for UUID
  "captureTime": 0("2023-10-27T14:30:15.123Z"),           // Tag 0 for RFC3339 string
  // Alternative: Represent reading as decimal fraction
  // "reading": 4([-3, -12345]),                          // Tag 4 for decimal fraction
  "reading": -12.345,                                     // Using standard float for simplicity here
  "readingScale": -3,                                     // Simple integer
  "rawValue": -12345,                                     // Simple integer
  "statusURL": 32("https://example.com/status/f81d4fae"), // Tag 32 for URI
  "alertPayload": h'01020304'                             // Direct byte string
  // Alternative: Use Tag 22 hint if JSON interop requires base64
  // "alertPayload": 22(h'01020304')
}
```

This example illustrates how tags integrate seamlessly into the CBOR structure. Tag 37 clearly identifies the `sensorID` bytes as a UUID, Tag 0 provides a standard string representation for `captureTime`, and Tag 32 marks the `statusURL` string as a URI. We chose to represent `reading` as a standard float, but Tag 4 could have been used for exact decimal precision if required by the application. For `alertPayload`, we used a direct byte string, as CBOR handles binary natively; Tag 22 could be added as a hint if this data frequently needs conversion to base64 for JSON compatibility. The tags add semantic precision and clarity beyond what the JSON representation alone could offer directly.

## Conclusion: The Power of Extensibility

CBOR Tags (Major Type 6) are the primary mechanism for extending CBOR's data model beyond its fundamental types. They provide a standardized way to imbue data items with specific semantic meaning, enabling the representation of complex types like dates, times, large or high-precision numbers, URIs, UUIDs, and much more, all while maintaining CBOR's characteristic compactness.1 The IANA registry plays a vital role in ensuring interoperability by providing a central authority for tag definitions, while the tiered registration system balances the need for stable, standardized core tags with flexibility for application-specific extensions.1

Understanding tags—how they work, where to find them, the implications of different ranges, and how to apply common ones—is key to leveraging the full power of CBOR. They allow engineers to model complex, meaningful data structures efficiently and in a way that promotes clarity and potential interoperability.

Looking ahead, tags are not just an isolated feature; they interact significantly with other advanced CBOR concepts:

- **Deterministic Encoding (dCBOR):** As we will explore later, achieving a canonical, byte-for-byte identical encoding for the same logical data requires strict rules. These rules apply to tags as well, mandating preferred serialization for tag numbers, and often requiring the consistent presence or absence of specific tags for certain semantic types. This is essential for applications like digital signatures or content-addressable storage where byte-level reproducibility is paramount.
- **Application Profiles (COSE, Gordian Envelope):** Many higher-level protocols and data formats built upon CBOR rely heavily on specific tags to define their structures and semantics. CBOR Object Signing and Encryption (COSE) uses tags extensively to identify signed, MACed, and encrypted messages and related security parameters. Similarly, the Gordian Envelope specification, which we will cover in detail later in this book, defines its own set of tags to structure its secure, layered data format. A solid grasp of CBOR tags is fundamental to working with these important application profiles.

Mastering CBOR tags moves us beyond simply encoding basic data structures towards building rich, extensible, and semantically precise data formats suitable for a wide range of applications, from constrained IoT devices to complex web protocols.
