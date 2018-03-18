[![](https://img.shields.io/crates/v/jbcrs-basic.svg)](https://crates.io/crates/jbcrs-basic) [![](https://docs.rs/jbcrs-basic/badge.svg)](https://docs.rs/jbcrs-basic)
# JBcRs-basic
**JBcRs is a Library, written in rust, to support reading and writing of java class files.**

If you want to read and write using a more *advanced* format, use [jbcrs](../README.md).
This library is not close from being finished,
but certain features have already been implemented:

- **Basic parsing:**
  A class file is parsed rather primitive:
  - You have access to the constant pool.
    Names, Descriptors and more are represented as `u16`,
    Indexing into the pool must be done manually.
    No validation of indices when parsing will be done.
  - Access Flags are decoded using the bitflags crate
    to provide a better experience.
  - Attributes are always parsed.
    This might be changed since it allows attackers
    to craft invalid Debug Attributes,
    which don't play a serious role in executing the code,
    Parsing the entire class file then might not work,
    since an error will be returned.
- **Basic writing:**
  A class file is written as it was parsed.

---
# Getting Started:

First, add this library as a dependency to your Cargo.toml
```toml
[dependencies]
jbcrs_basic = "0.1.4"
```

We want to parse a class from a byte array
and print its version, access flags and name.
Of course you could use std::fs::File or a zip library,
but showing this is not the purpose of this tutorial.

```rust
extern crate jbcrs_basic;

use jbcrs_basic::*;

// You got the bytes from any possible source.
let bytes: &[u8] = [0xCA, 0xFE, 0xBA, 0xBE];

// After parsing the class file,
// you will get the constant pool
// and the class itself.
// You don't have to annotate the types here.
let (constant_pool, class): (Pool, Class) = parse(bytes)
    .expect("could not parse class file");

// Print its major and minor version:
println!("version: {}.{}", class.major_version, class.minor_version);

// Access Flags can be printed human readable
println!("access: {:?}", class.access_flags);

// Printing the name requires us to use the constant pool.
println!("name: {}", constant_pool.get_class_name(class.name).expect("could not get class name"));
```

---
# Resources
[Java Virtual Machine Specification (Java SE 9)](https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf)
