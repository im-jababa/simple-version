# simple-version

[![Github](https://img.shields.io/badge/github-foolkat/simple_version-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/foolkat/simple-version)
[![Crates.io](https://img.shields.io/crates/v/simple-version.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/simple-version)
[![Docs.rs](https://img.shields.io/badge/docs.rs-simple_version-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/simple-version)
[![Build](https://img.shields.io/github/actions/workflow/status/foolkat/simple-version/rust.yml?branch=main&style=for-the-badge)](https://github.com/foolkat/simple-version/actions?query=branch%3Amain)

A small Rust library that provides a `Version<T>` type for handling semantic versioning, 
including an optional build number. It also offers a `version_from_pkg!` macro to embed
the crate's compile-time `CARGO_PKG_VERSION` directly into your code.

---

## Examples

### 1. create a new `Version` object
*(without a build number)*

```rust
use simple_version::Version;

let v = Version::<u32>::new(1, 2, 3);

println!("{}", v);  // 1.2.3
```

---

### 2. create a new `Version` object
*(with a build number)*

```rust
use simple_version::Version;

let v = Version::<u32>::new(1, 2, 3).build(4);

println!("{}", v);  // 1.2.3+4
```

---

### 3. create a new `Version` object
*(using the Cargo package version)*

```toml
# Cargo.toml

[package]
name = "..."
version = "1.2.3"
...
```

```rust
use simple_version::{Version, version_from_pkg};

let v: Version<u32> = version_from_pkg!(u32);

println!("{}", v);  // 1.2.3
```

---

### 4. Compare between two versions

```rust
use simple_version::Version;

let v1 = Version::<u32>::new(1, 999, 999);
let v2 = Version::<u32>::new(2, 0, 0);

assert!(v1 < v2);
```

---

### 5. Compare between two versions
*(with exceptions)*

```rust
use simple_version::Version;

let v1 = Version::<u32>::new(1, 0, 0).build(1); // v1 has a build number.
let v2 = Version::<u32>::new(1, 0, 0);          // v2 does not.

assert!(v1 > v2);   // In this case, v1 is greater.
```