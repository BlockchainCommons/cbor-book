# The CBOR, dCBOR, and Gordian Envelope Book

## Toolchain

- This book is being developed with the `mdBook` tool, with the actual text of the book in `src/` as markdown files.

## Writing Guidelines

- All code examples are in Rust, using the API in the `dcbor` and `bc-envelope` crates in this workspace.

---

## Book Outline

- Below the detailed outline for this e-book.
- The structure reflects a logical progression from foundational knowledge (CBOR), through deterministic structure (dCBOR), to applied composability and semantics (Gordian Envelope).
- It is designed to be practical and accessible for software developers, with a focus on real-world applications and examples.
- Knowledge of Rust and familiarity with serialization formats such as JSON or Protobuf is assumed.
- We are not currently numbering chapters, as their number and order may change as we write the book.
- The book is divided into three main parts:
  - **Part I: CBOR** — Introduction to the Concise Binary Object Representation format.
  - **Part II: dCBOR** — A deterministic variant of CBOR for cryptographic applications.
  - **Part III: Gordian Envelope** — A composable structure for cryptographic documents and assertions.

# **CBOR, dCBOR, and Gordian Envelope: A Practical Guide for Software Developers**

## **Preface**

- Why this book?
- Who this book is for
- Prerequisites and assumptions
- Code examples and tooling
- Acknowledgements

---

## **Part I: CBOR — Concise Binary Object Representation**

### **Chapter: Introduction to CBOR**
- What is CBOR and why does it exist?
- Comparison to JSON and other serialization formats (e.g., MessagePack, Protobuf)
- Key design goals: compactness, efficiency, schema-optional, extensibility

### **Chapter: CBOR Data Model**
- Core data types (major types 0–7)
  - Integers, byte strings, text strings
  - Arrays, maps
  - Tags and simple values
- Major type encoding and wire format structure
- Indefinite-length items
- Semantic tagging

### **Chapter: Using CBOR in Practice**
- CBOR libraries in major languages
- Encoding and decoding examples
- Performance considerations
- Common pitfalls and interoperability gotchas

### **Chapter: Determinism and Canonicalization in CBOR**
- Why determinism matters (e.g., hashing, signing, reproducibility)
- Existing canonicalization standards (CTAP2, DAG-CBOR, etc.)
- Challenges in achieving reliable deterministic CBOR
- Negative zero, NaNs, float precision — where CBOR wavers

---

## **Part II: dCBOR — Deterministic CBOR**

### **Chapter: Motivation for dCBOR**
- The need for stricter determinism
- Use cases: content-addressable storage, signatures, consensus protocols
- Where canonical CBOR fails or is too ambiguous

### **Chapter: The dCBOR Specification**
- Key constraints and reductions
  - No indefinite lengths
  - Lexical map key ordering
  - Floating-point normalization and numeric reduction
  - Disallowed encodings (e.g., negative zero)
- Deterministic tag handling
- Deterministic encoding and decoding

### **Chapter: Implementing dCBOR**
- Existing libraries and implementations (Rust, Swift, C/C++)
- Encoding pipeline: CBOR → dCBOR reduction → serialization
- dCBOR round-trip guarantees
- Integration into new and legacy codebases

### **Chapter: Testing and Validation**
- Verifying conformance to dCBOR rules
- Hash comparison and reproducibility tests
- Tooling for test vectors and test suites

---

## **Part III: Gordian Envelope — Composable Cryptographic Documents**

### **Chapter 9: Introduction to Gordian Envelope**
- What is an Envelope?
- Use cases: digital credentials, decentralized identity, cryptographic attestations, provenance, access control
- Why CBOR + dCBOR was necessary, but not sufficient

### **Chapter 10: Envelope Semantics and Structure**
- Core model: assertions, subjects, predicates, and objects
- Tree-based composition: envelopes inside envelopes
- Assertions as nested documents
- Support for elision, encryption, and signing

### **Chapter 11: Envelope Encoding and Processing**
- Envelope as a CBOR superstructure
- Signing and verification using Signature-with-Metadata
- Encryption patterns (envelope wrapping, encrypted assertions)
- Elision for selective disclosure and privacy-preserving documents

### **Chapter 12: Practical Applications and Patterns**
- Examples:
  - Verifiable claims (Person → Employment → Company)
  - Decentralized identifiers (DIDs and Envelopes)
  - Smart contract commitments and receipts
  - Document provenance and audit trails
- Interoperability with C2PA, JOSE, and JSON-LD

### **Chapter 13: Tooling and Libraries**
- Reference libraries (Envelope library, dCBOR encoders)
- Envelope CLI and JSON diagnostic tools
- Integrating with your application's trust model

### **Chapter 14: Advanced Topics**
- Envelope in distributed systems
- Data provenance chains and object capabilities
- Composability and recursive validation
- Future directions: zero-knowledge, selective disclosure, encrypted graphs

---

## **Appendices**

### **Appendix A: CBOR Diagnostic Notation Cheat Sheet**
- Examples and syntax

### **Appendix B: dCBOR Reduction Examples**
- Before and after reduction

### **Appendix C: Envelope DSL Overview**
- Human-readable structure of envelopes for developers

### **Appendix D: Tag Registry**
- IANA tags used
- Private tag ranges for applications

### **Appendix E: Glossary**
- Definitions of key terms

### **Appendix F: Resources**
- Links to specifications, GitHub repositories, mailing lists, and community tools
