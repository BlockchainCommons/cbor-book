# CBOR vs. The Other Guys

## The Binary Serialization Landscape

In the previous chapter, we traced data interchange formats from verbose XML to simpler JSON, highlighting the quest for better ways to represent and exchange data. JSON's simplicity and performance advantages over XML made it dominant for web APIs. However, its text-based nature limits efficiency in size and processing speed. This led to CBOR (Concise Binary Object Representation), which retains JSON's familiar data model while leveraging binary encoding for compactness and performance—crucial for constrained environments like the Internet of Things (IoT).

CBOR exists within a broader landscape of binary serialization formats, each with specific goals and trade-offs. Understanding how CBOR compares to alternatives helps appreciate its strengths and make informed format decisions. This chapter surveys several prominent binary formats:

- [BSON (Binary JSON)](https://bsonspec.org/): Developed by MongoDB for internal storage and wire format, extending JSON with database-centric types and optimizing for traversability.

- [Protocol Buffers (Protobuf)](https://protobuf.dev/): Google's high-performance, schema-driven format designed for efficient Remote Procedure Calls and data archival.

- [MessagePack](https://msgpack.org/): A fast, compact binary alternative to JSON, used for network communication and caching.

- [Avro](https://avro.apache.org/): An Apache project emphasizing robust schema evolution, common in big data ecosystems like Hadoop and Kafka.

We'll compare these formats based on origins, data models, encoding strategies, schema approaches, performance characteristics, extensibility mechanisms, and typical use cases.

A fundamental distinction exists between **schema-optional** formats (CBOR, BSON, MessagePack) and **schema-driven** formats (Protocol Buffers, Avro). Schema-optional formats embed type information with the data, allowing parsing without prior structural knowledge—like JSON. This offers flexibility but introduces overhead and may require runtime validation. Schema-driven formats rely on external schemas known by both sender and receiver, potentially enabling more compact encodings (omitting field names/types) and compile-time validation, but requiring schema management and reducing data self-description. This core difference often reflects each format's origin—whether designed for flexible document storage like BSON or for high-performance, predefined message structures like Protobuf.

## BSON: Binary JSON Tailored for Databases

BSON emerged directly from MongoDB's needs. While MongoDB embraced JSON's flexible document model, raw JSON proved suboptimal for database operations due to its limited type system (lacking dates and binary data) and inefficiencies when parsing text for queries and indexing.

MongoDB created BSON to address these limitations—providing a binary JSON representation optimized for storage efficiency, rapid traversal, and enhanced type support while preserving schema flexibility. BSON serves as MongoDB's native format for both storage and network transfer.

### Design and Encoding

BSON documents serialize as binary data with explicit type and length information. Each document begins with a 4-byte total size, followed by a sequence of elements, ending with a null byte. Each element contains a one-byte type code, null-terminated field name, and type-specific value encoding. The inclusion of length prefixes enables MongoDB to quickly traverse documents and access specific fields without parsing entire structures.

BSON extends JSON's data model with several database-essential types:

- **ObjectId:** A 12-byte unique identifier (timestamp + machine ID + process ID + counter), commonly used as primary key.

- **Date:** 64-bit integer representing milliseconds since Unix epoch.

- **Binary Data (BinData):** Direct embedding of byte arrays with subtype indicators, avoiding Base64 encoding.

- **Timestamp:** Special 64-bit type (seconds since epoch + ordinal) for MongoDB replication logs.

- **Additional Numeric Types:** 32-bit integers (`int32`), 64-bit integers (`int64`), 64-bit floats (`double`), and 128-bit high-precision decimals (`Decimal128`) for financial applications.

- **Deprecated Types:** Including `Undefined` (generally discouraged).

A notable design choice is BSON's array encoding—represented as standard BSON documents with string keys matching array indices ("0", "1", "2"). While simplifying internal representation (everything is a document), this adds overhead compared to more efficient array encodings.

BSON prioritizes traversability and in-place updates. Length prefixes enable field skipping during reads, while fixed-size numeric encodings simplify value modification without rewriting entire documents.

### Pros and Cons

BSON's primary strengths derive from its MongoDB integration. It enables faster document traversal than parsing JSON text, with richer data types (dates, binary data, ObjectIds, high-precision decimals) essential for database operations. It maintains JSON's schema flexibility while allowing MongoDB to build indexes on document fields for efficient querying.

However, BSON has notable limitations. Type and length prefixes, along with verbose array encoding, often make BSON documents larger than equivalent JSON, particularly for small documents. It's generally less space-efficient than MessagePack or Protobuf. Like most binary formats, it lacks human readability. Its extended types prevent lossless conversion to standard JSON, limiting interoperability. BSON remains largely confined to the MongoDB ecosystem and lacks built-in RPC mechanisms.

### Comparison vs. CBOR

Both CBOR and BSON are schema-optional binary formats extending the JSON data model, but with different design priorities. BSON optimizes for database storage and traversal, using length prefixes and specialized types like `ObjectId` and `Decimal128`, sometimes sacrificing compactness. CBOR prioritizes conciseness and implementation simplicity for network transmission in constrained environments, typically achieving smaller message sizes. While BSON offers database-centric types, CBOR employs a more general type system extended through standardized tags (for dates, bignums, etc.). BSON remains closely tied to MongoDB, whereas CBOR exists as an IETF standard (RFC 8949) used across various internet protocols.

BSON's design clearly reflects its purpose as MongoDB's internal format. The need for rapid field access drove the inclusion of length prefixes, while database requirements dictated specialized types like `Date`, `BinData`, and `ObjectId`. These adaptations make BSON more than just binary JSON—it's an extended format tailored for database operations. This specialization benefits MongoDB but creates trade-offs in size and general interoperability compared to formats designed for broader use cases. The term "Binary JSON" can therefore be somewhat misleading, as its extended types prevent guaranteed lossless round-tripping with standard JSON.

## Protocol Buffers: Schema-Driven Performance

Protocol Buffers (Protobuf) originated at Google as a mechanism for serializing structured data, designed to be smaller, faster, and simpler than XML. Initially created for internal RPC and data storage, Google open-sourced it in 2008.

### Design and Encoding

Protobuf takes a fundamentally **schema-driven** approach. Data structures ("messages") must be defined in `.proto` files using Protobuf's Interface Definition Language (IDL).

The workflow centers on the `protoc` compiler, which processes `.proto` files to generate source code in various languages (C++, Java, Python, Go, C#, etc.). This generated code provides message classes with methods for field access, serialization, and parsing.

The binary format prioritizes compactness and speed. Instead of field names, each field uses a unique **field number** (tag) paired with a **wire type** indicating the encoding method. Wire types specify how much data to read (e.g., `VARINT` for variable-length integers, `LEN` for length-delimited data like strings).

Encoding techniques include **Varints** (using fewer bytes for smaller numbers) and **ZigZag encoding** (for efficient negative number representation). The data model supports numerous scalar types (`int32`, `uint64`, `bool`, `string`, etc.), nested messages, `repeated` fields (arrays), `map` fields (key-value pairs), and `oneof` (mutually exclusive fields).

Protobuf handles **schema evolution** well. As long as field numbers remain stable, developers can typically add or remove optional/repeated fields without breaking compatibility. Parsers skip unknown fields, enabling forward compatibility. However, changing field types is generally unsafe, and using `required` fields (discouraged in newer versions) limits evolution flexibility.

### Pros and Cons

Protobuf's advantages derive from its schema-driven approach, delivering high performance with compact message sizes by replacing field names with numeric tags. The schema and generated code provide compile-time type safety and simplified data access. Its evolution capabilities allow systems to change without breaking compatibility. Language-neutral code generation suits polyglot environments.

However, these schema requirements create notable limitations. Protobuf data isn't self-describing—the `.proto` definition is essential for interpreting the binary data. The format isn't human-readable. The workflow requires compiling `.proto` files and managing generated code, reducing flexibility for dynamic data structures. It can be suboptimal for very large messages (over a few megabytes) or multi-dimensional numeric arrays common in scientific computing. While widely adopted, Protobuf lacks formal standardization by bodies like IETF or W3C.

### Comparison vs. CBOR

The fundamental difference is their schema approach. Protobuf mandates schemas (`.proto` files) and compilation. CBOR is **schema-optional** with self-describing data containing embedded type indicators. While CBOR supports validation with schema languages like [CDDL](https://datatracker.ietf.org/doc/html/rfc8610), schemas aren't required for basic parsing.

This creates distinctions in self-description (CBOR yes, Protobuf no), encoding strategy (CBOR uses type indicators with string map keys; Protobuf uses numeric field tags and wire types), flexibility (CBOR higher, Protobuf more rigid but safer), and extensibility (CBOR uses IANA-registered tags, Protobuf uses `.proto`-defined options/extensions).

Performance comparisons are nuanced. Protobuf excels in speed and size, particularly for RPC with pre-shared schemas. CBOR also prioritizes efficiency, especially minimizing codec size for constrained devices. Results depend heavily on data, implementation quality, and use case. For standardization, CBOR is an IETF standard (RFC 8949), while Protobuf remains a Google-driven de facto standard.

Protobuf's philosophy achieves performance, compactness, and type safety through mandatory schemas and code generation—highly effective in controlled environments where schema management is feasible. This tight coupling yields efficiency gains but sacrifices the flexibility and self-description offered by formats like JSON or CBOR. The trade-off is clear: Protobuf prioritizes performance and structural rigidity, whereas CBOR favors flexibility and self-description while maintaining binary efficiency.

## MessagePack: The Compact JSON Alternative

MessagePack emerged around 2008-2009, created by Sadayuki Furuhashi. Its goal was to provide a more efficient binary serialization format than JSON – "like JSON, but fast and small." It addresses scenarios where JSON's verbosity creates bottlenecks, such as network communication (RPC, message queues) and data caching (e.g., in Memcached).

### Design and Encoding

MessagePack defines a binary format mirroring JSON's fundamental data types (null, boolean, integer, floating-point, string, array, map) while enabling transparent conversion between formats.

Beyond JSON types, MessagePack adds:

- **`bin` (Binary Data):** Efficient storage for raw byte sequences.

- **`ext` (Extension Type):** Mechanism for application-specific types, consisting of an integer type code (tag) and a byte string payload.

The encoding prioritizes compactness. Small integers can be encoded in a single byte. Short strings need only a length prefix followed by UTF-8 bytes. Arrays and maps include their element count as a prefix. Unlike JSON, MessagePack allows any data type as map keys, not just strings. Data types and lengths are indicated by initial encoded bytes.

### Pros and Cons

MessagePack delivers greater efficiency than JSON through smaller serialized output, optimized type encodings, potentially faster network transmission, and reduced storage requirements. Serialization and deserialization can outperform standard JSON libraries, though actual performance depends on implementations and data characteristics. It supports native binary data with an extension mechanism for custom types and offers implementations across numerous programming languages.

However, MessagePack sacrifices human-readability, complicating debugging. A significant limitation affects streaming: since arrays and maps require upfront element counts, streaming serialization becomes difficult when total counts aren't known in advance, potentially requiring complete in-memory buffering. While often faster than JSON, the margin varies with implementation quality and optimization. Compared to CBOR, MessagePack lacks formal standardization through bodies like IETF (its specification resides on GitHub), and its `ext` mechanism provides less structure than CBOR's IANA-registered tags.

### Comparison vs. CBOR
CBOR and MessagePack both aim to be efficient, schema-less binary alternatives to JSON with native binary data support, but differ in key aspects:

- **Encoding Details:** CBOR supports indefinite-length arrays and maps (beneficial for streaming when total size is unknown), while MessagePack typically requires fixed collection counts.

- **Standardization:** CBOR is a formal IETF standard (RFC 8949) developed through consensus, whereas MessagePack uses a community-maintained specification. Many view CBOR as a more rigorous standard inspired by MessagePack.

- **Extensibility:** CBOR employs a standardized semantic tag system with an IANA registry for extended types (dates, URIs, bignums). MessagePack uses a simpler but less structured `ext` type where applications define tag meanings.

- **Performance and Size:** Comparisons vary by implementation and data. CBOR prioritizes small codec size (for constrained devices) alongside message compactness, while MessagePack focuses primarily on message size and speed.

- **Conceptual Simplicity:** MessagePack's shorter specification appears simpler, but CBOR's unification of types under its major type/additional info system and tag mechanism offers conceptual clarity.

MessagePack pioneered the "binary JSON" concept to improve network performance, optimizing for complete, known data structures rather than streaming scenarios. Its widespread adoption demonstrates market demand. However, CBOR's formal standardization, streaming support through indefinite-length items, and standardized tag registry target broader applications, particularly for constrained devices and internet protocols.

## Avro: Mastering Schema Evolution

Apache Avro emerged from Apache Hadoop around 2009, designed specifically to address schema evolution challenges in large-scale data processing systems. In environments like Hadoop or Kafka data pipelines, where producers and consumers evolve independently, Avro enables seamless schema changes without breaking compatibility. It offers rich data structures and integrates easily with dynamic languages, without requiring code generation.

### Design and Encoding

Avro is **schema-based**, with schemas typically defined in JSON (though an alternative C-like IDL is available). A fundamental aspect of Avro is that the schema used to _write_ data is always required to _read_ that data. The binary encoding contains no field names or type identifiers—just concatenated field values in schema-defined order. This creates compact data that depends entirely on the schema for interpretation. Writer schemas typically accompany the data in file headers or through schema registry services. Avro also supports JSON encoding for debugging purposes.

Avro includes primitive types (`null`, `boolean`, `int`, `long`, `float`, `double`, `bytes`, `string`) and complex types (`record`, `enum`, `array`, `map`, `union`, `fixed`). Records contain named fields, arrays hold sequences, maps store key-value pairs (string keys only), and unions allow values of several specified types—commonly used for optional fields by including `null` (e.g., `["null", "string"]`).

Avro's strength lies in its well-defined **schema evolution rules**:

- Fields can be added or removed only if they have a default value, which readers use when the field is missing.
- Field renaming uses `aliases` in the reader's schema to recognize data written with old names.
- Type changes are generally forbidden, with limited exceptions (e.g., `int` to `long`).
- For enums, adding symbols is backward compatible; removing or renaming breaks compatibility.

When reading data with a different but compatible schema, Avro uses **schema resolution**—comparing field names (and aliases) and applying defaults to present data according to the reader's schema.

### Pros and Cons

Avro's main advantage is sophisticated schema evolution handling, making it ideal for systems with frequent or independent schema changes. JSON-defined schemas are relatively easy to manage. The binary encoding is compact since it omits field names and tags. Avro integrates well with dynamic languages when schemas are available at runtime. It has strong adoption within the Apache ecosystem, particularly Hadoop, Spark, and Kafka.

The primary disadvantage is requiring the writer's schema during deserialization, introducing schema management complexity and often necessitating a schema registry. While compact, some benchmarks suggest Avro may be slower than Protobuf in certain scenarios. The binary format is not human-readable, and developers must carefully follow schema evolution rules to maintain compatibility.

### Comparison vs. CBOR

Avro and CBOR represent fundamentally different schema philosophies. Avro _requires_ schemas for reading and writing, with design centered on schema resolution. CBOR is **schema-optional** and self-describing; schemas (like CDDL) can validate but aren't needed for parsing.

This affects encoding: Avro omits field identifiers, relying on schema field order. CBOR includes type information and map keys, making it interpretable without external schemas.

Avro handles schema evolution explicitly through resolution rules, defaults, and aliases. CBOR's self-describing nature allows parsers to skip unknown data, but complex changes may require application-level logic or tag conventions. CBOR offers greater ad-hoc flexibility, while Avro enforces structure through schemas. Their ecosystems also differ—Avro dominates Big Data/Apache contexts, while CBOR prevails in IoT and IETF protocols.

Avro's design clearly optimizes for schema evolution in large-scale, long-lived data systems. By requiring the writer's schema at read time, it enables powerful resolution capabilities, allowing independent producer and consumer evolution. This contrasts with Protobuf's reliance on stable tag numbers and CBOR's schema-optional flexibility. The trade-off is explicit: Avro gains robust evolution and dynamic language integration, but requires schema management and produces data that's not self-contained.

## Comparative Analysis: Choosing the Right Tool

Having examined several binary serialization formats, it's clear that each addresses specific needs in the data interchange landscape. BSON optimizes for MongoDB's database operations. Protocol Buffers achieves high performance and type safety for RPC through mandatory schemas. MessagePack provides a compact binary alternative to JSON for network communication. Avro specializes in managing schema evolution for data pipelines. CBOR offers a standardized, binary-efficient encoding of the JSON data model with emphasis on constrained environments and extensibility.

No single format suits all use cases. The optimal choice depends on specific application requirements. Key decision factors include schema requirements (mandatory vs. optional), performance needs vs. flexibility, schema evolution complexity, ecosystem compatibility, and specialized features like native data types or standardized extensibility mechanisms.

The following table summarizes the key distinctions between these formats:

| Feature                 | CBOR                                           | BSON                             | Protocol Buffers                   | MessagePack                               | Avro                                     |
| :---------------------- | :--------------------------------------------- | :------------------------------- | :--------------------------------- | :---------------------------------------- | :--------------------------------------- |
| **Origin/Primary Goal** | IETF / Constrained Env Efficiency              | MongoDB / DB Storage & Traversal | Google / RPC Performance & Size    | Furuhashi / JSON Alternative (Speed/Size) | Apache / Schema Evolution                |
| **Schema Handling**     | Optional                                       | Optional                         | Required (`.proto` IDL)            | Optional                                  | Required (JSON or IDL)                   |
| **Schema Location**     | N/A or Separate (e.g., CDDL)                   | N/A                              | Separate (`.proto` file)           | N/A                                       | With Data (Files) or Registry            |
| **Self-Describing?**    | Yes                                            | Yes                              | No                                 | Yes                                       | No (Binary requires schema)              |
| **Encoding Basis**      | JSON Model + Tags                              | Extended JSON Model              | Schema Tags/Numbers                | JSON Model + `ext` type                   | Schema Field Order                       |
| **Extensibility**       | IANA Tags                                      | Custom Types (DB-centric)        | Proto Extensions/Options           | `ext` type                                | Schema Evolution Rules                   |
| **Schema Evolution**    | Implicit (Tags/Skipping)                       | Implicit                         | Explicit (Tag Stability)           | Implicit (`ext`/Skipping)                 | Explicit (Resolution, Defaults, Aliases) |
| **Typical Size**        | Compact                                        | Variable (can be large)          | Very Compact                       | Compact                                   | Compact (Binary)                         |
| **Typical Speed**       | Fast (esp. constrained codec)                  | Fast Traversal (DB context)      | Very Fast (RPC context)            | Fast                                      | Fast                                     |
| **Standardization**     | IETF RFC 8949                                  | De facto (MongoDB)               | De facto (Google)                  | Community Spec                            | Apache Project                           |
| **Primary Use Cases**   | IoT, CoAP, COSE, Security, Deterministic Needs | MongoDB                          | RPC, Microservices, Internal Comms | Network Comms, Caching, RPC               | Big Data (Hadoop, Kafka), Data Pipelines |

_Note: Size and speed comparisons are general tendencies; actual performance depends heavily on data structure, implementation quality, and specific workload._

This comparison highlights the complex trade-offs between formats. Protocol Buffers excels when validation, compactness, and RPC performance are critical in environments where schema management is feasible. Avro offers superior schema evolution capabilities for large-scale data pipelines, despite requiring schema distribution mechanisms. BSON serves specialized needs within the MongoDB ecosystem. MessagePack provides an efficient binary alternative to JSON for network communication, though with potential streaming limitations. CBOR stands out when IETF standardization, constrained device support, binary-efficient JSON encoding, standardized extensibility, or deterministic encoding are priorities.

## Why Choose CBOR?

Based on the preceding comparisons, CBOR presents a unique combination of features that make it the preferred choice in several specific contexts:

- **JSON Data Model Fidelity in Binary:** CBOR provides a direct binary encoding for the familiar JSON data model. This lowers the adoption barrier for developers already comfortable with JSON, unlike formats requiring different structural concepts or mandatory schemas.

- **Efficiency for Constrained Environments:** CBOR was explicitly designed for the Internet of Things and constrained environments. This yields encoders and decoders with small code footprints, efficient processing, and significantly reduced message sizes compared to JSON—all critical for resource-limited devices.

- **IETF Standardization and Integration:** As an IETF standard (RFC 8949), CBOR benefits from rigorous review and a stable specification. It integrates within the broader internet protocol ecosystem, serving as a payload format in CoAP and forming the basis of COSE (CBOR Object Signing and Encryption), crucial for security in constrained environments.

- **Standardized Extensibility via Tags:** CBOR includes a well-defined mechanism for extending the basic data model using semantic tags. These IANA-registered tags provide standardized ways to represent richer semantics while allowing basic decoders to skip tags they don't understand. This offers a more structured approach than MessagePack's `ext` type.

- **Schema-Optional Flexibility:** CBOR remains schema-optional like JSON. Data is self-describing, allowing for parsing without predefined schemas—advantageous for evolving systems or ad-hoc data exchange. When validation is needed, external schema languages like CDDL (RFC 8610) can be employed.

- **Native Binary Data Support:** CBOR includes a native byte string type, allowing efficient representation of binary data without inefficient text encodings like Base64 required by JSON.

- **Deterministic Encoding Potential:** RFC 8949 Section 4.2 explicitly defines rules for deterministic encoding, ensuring the same data structure always serializes to identical byte sequences—critical for cryptographic applications where reproducibility is essential.

While CBOR offers these advantages, it's not human-readable like JSON. In high-performance RPC scenarios with fixed schemas, optimized Protobuf implementations might offer better raw performance. Though its ecosystem is growing, particularly in IoT and security domains, it might not have the breadth of tooling found for JSON or Protobuf in every application area.

CBOR occupies a compelling position in the serialization landscape—a standardized, extensible, and efficient binary format built on the widely understood JSON data model. Its design for constrained environments, IETF protocol integration, and support for deterministic encoding make it well-suited for IoT, secure communication, and verifiable data structures, all without imposing the mandatory schemas found in Protocol Buffers or Avro.

## CBOR as a Foundation for Blockchain Commons

Blockchain Commons' specifications, including dCBOR (Deterministic CBOR) and Gordian Envelope, build directly on CBOR primarily due to its deterministic encoding capabilities.

Gordian Envelope, a "smart documents" format containing cryptographic material like keys and verifiable credentials, relies on cryptographic hashing for data integrity and selective disclosure. These functions require deterministic serialization—identical semantic data must produce identical byte sequences when encoded.

CBOR's RFC 8949 explicitly defines a "Deterministically Encoded CBOR" profile that mandates preferred integer encodings and lexicographically ordered map keys. This standardized approach to determinism gives CBOR a significant advantage over JSON (which lacks universal canonicalization) and other binary formats where determinism isn't prioritized.

While RFC 8949 established deterministic guidelines, Blockchain Commons identified remaining ambiguities that could lead to inconsistent implementations. Their dCBOR application profile, documented as an IETF Internet-Draft, further refines these rules by rejecting duplicate map keys and establishing precise numeric reduction rules to ensure values like 10, 10.0, and 10.00 encode identically.

Beyond determinism, CBOR offered additional advantages: structured binary representation suitable for cryptographic data, conciseness, standardized tag-based extensibility, IETF standardization, compatibility with constrained environments (important for hardware wallets), and platform independence.

CBOR thus provided the standardized deterministic foundation that Blockchain Commons refined through dCBOR to build secure, interoperable systems like Gordian Envelope—topics covered in later chapters.

## Conclusion: A Diverse Binary Ecosystem

The evolution from XML to binary formats like BSON, Protocol Buffers, MessagePack, Avro, and CBOR reflects a landscape where no single "best" serialization format exists. Each represents specific design choices optimized for particular contexts.

- **BSON** prioritizes efficient storage and traversal in MongoDB, extending JSON with specialized types at the cost of compactness and broader interoperability.
- **Protocol Buffers** achieves performance and compactness for RPC through mandatory schemas and code generation, trading flexibility and self-description.
- **MessagePack** offers a compact binary JSON alternative for network communication, despite potential streaming limitations.
- **Avro** excels at schema evolution in data pipelines, requiring schema availability but providing robust compatibility features.
- **CBOR** delivers an IETF-standardized, binary-efficient JSON encoding that balances flexibility with performance, offering standardized extensibility and serving constrained environments and deterministic encoding needs.

These diverse formats will continue to coexist, with developers selecting tools that match their project requirements. CBOR's position as a standardized, efficient format based on the JSON model ensures its relevance, particularly for IoT, secure systems, and verifiable data structures.
