# Introduction

## TL;DR: Who is This Book For?

- If your primary goal is to understand a modern, efficient binary serialization format that offers significant performance and size benefits over JSON, Part I provides a comprehensive guide to CBOR.

- If your application requires absolute, verifiable consistency – for digital signatures, content hashing, consensus, or interoperable verification – Part II delves into the principles of determinism and the specifics of dCBOR, including a tutorial for the `dcbor` Rust crate.

- For those building applications that require structured, verifiable, and privacy-preserving data – _smart documents_ – Part III explores the groundbreaking capabilities of Gordian Envelope, including usage of the `bc-envelope` Rust crate.


---


## Navigating the Landscape of Modern Data Representation

Modern software engineering demands tools that can handle the dual pressures of performance and trust. As systems grow more distributed and data security becomes paramount, developers must move beyond formats that were designed for a simpler era. JSON, while readable and ubiquitous, can introduce unacceptable inefficiencies—slowing performance, bloating payloads, and leaving room for ambiguity where certainty is required.

This book introduces a progressive technological stack—CBOR, dCBOR, and Gordian Envelope—that addresses these modern challenges head-on. Together, they form a path from compact binary encoding to cryptographically verifiable and privacy-preserving data structures. Understanding these tools enables engineers and decision-makers to build faster, leaner, and more trustworthy systems—without compromise.


## Part I: The Foundation – Achieving Efficiency with CBOR

At the core of this stack is CBOR: Concise Binary Object Representation, defined in RFC 8949. It was designed for constrained environments, enabling implementations with minimal memory and CPU usage—ideal for IoT devices, embedded systems, and high-throughput applications.

CBOR offers significant efficiency over JSON. Its binary encoding means faster parsing, smaller messages, and lower latency. It also supports an extended data model, including binary byte strings, which simplifies integration with existing JSON systems while enabling more advanced use cases.

Crucially, CBOR is extensible. New tags can be introduced without breaking older implementations, reducing long-term maintenance burdens. Protocols can evolve without costly version negotiations, and features can be rolled out faster and more safely.

While JSON emphasizes human readability, CBOR prioritizes performance, resource efficiency, and future-proof extensibility. In many systems—especially where network bandwidth, power, or processing time is scarce—CBOR isn’t just better. It’s necessary.


## Part II: The Guarantee – Ensuring Verifiable Consistency with dCBOR

Efficiency alone isn’t enough for systems that rely on trust. Distributed ledgers, digital signatures, and content-addressed data all depend on one principle: the exact same data must always serialize to the exact same bytes.

CBOR provides guidelines for deterministic encoding—but leaves enough leeway to produce different byte sequences for the same logical structure. This is a problem for cryptography and consensus protocols, where even one byte of variance invalidates signatures or breaks agreement.

Deterministic CBOR (dCBOR) solves this. It is a strict profile of CBOR that eliminates ambiguity. It defines canonical rules for numeric encoding (e.g., converting `2.0` to `2`, collapsing NaNs into a single representation), mandates lexicographic sorting of map keys by their encoded form, and forbids features like indefinite-length values that undermine determinism.

dCBOR isn’t a fork. It’s fully valid CBOR, but with stricter rules and mandatory validation. Encoders must produce canonical output. Decoders must reject anything that isn’t. This guards against inconsistency, manipulation, and protocol divergence—critical when trust is on the line.

For engineering teams building security-critical systems, dCBOR provides the byte-level reliability needed to anchor hashing, signing, auditing, and cross-platform integrity checks. It replaces ambiguity with assurance.


## Part III: The Breakthrough – Secure, Structured, Privacy-Enhancing Data with Gordian Envelope

Once determinism is in place, it becomes possible to build something far more powerful: a structured, secure, and privacy-aware data format that can adapt to the demands of modern identity, privacy, and trust. That’s what Gordian Envelope delivers.

Built atop dCBOR, Gordian Envelope enables deeply structured data with built-in cryptographic integrity. It’s a semantic format—often modeled as subject-predicate-object triples—wrapped in a Merkle-like digest tree. This structure guarantees that every element, not just the whole, can be independently verified.

What sets Envelope apart is **holder-controlled elision**: the ability to redact or hide portions of the data without invalidating the overall structure or breaking attached signatures. This enables minimal disclosure, progressive trust, and user-controlled privacy—foundational principles for self-sovereign identity and modern data sovereignty.

Envelope also supports advanced layering: encryption, compression, nested signatures, and semantic annotations. These features don’t just bolt on—they integrate directly with the underlying structure, allowing powerful capabilities like verifiable redaction, authenticated subtrees, and selective disclosure proofs.

Its use cases are broad and high-impact: verifiable credentials, digital wallets, secure logs, privacy-preserving data sharing, and cryptographic asset management. More fundamentally, Envelope shifts control from institutions to individuals. Data no longer belongs solely to the issuer. It’s held, managed, and selectively revealed by the user.

This is the architecture of trust, built from the bottom up: efficient encoding, deterministic consistency, and cryptographic structure, all aligned with privacy and user agency.

CBOR is the foundation. dCBOR is the guarantee. Gordian Envelope is the future.
