[![](https://img.shields.io/crates/v/jbcrs.svg)](https://crates.io/crates/jbcrs) [![](https://docs.rs/jbcrs/badge.svg)](https://docs.rs/jbcrs)
# JBcRs
**JBcRs is a Library, written in rust, to support reading and writing of java class files.**

If you want to read and write using a *default* format, use [jbcrs_basic](basic/README.md).
This library is not close from being finished,
but certain features have already been implemented:

- **Parsing and writing of Field- and Methoddescriptors**

---
# Getting Started:

First, add this library as a dependency to your Cargo.toml
```toml
[dependencies]
jbcrs = "0.1.3"
```

---
# Resources
[Java Virtual Machine Specification (Java SE 9)](https://docs.oracle.com/javase/specs/jvms/se9/jvms9.pdf)
