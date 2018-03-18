use std::{io, result};

#[derive(Debug, YadeError)]
pub enum Error {
    /// Just an I/O error
    IO(#[cause] io::Error),

    /// A modified utf-8 string could not be read
    InvalidUTF8,

    /// Decoder has come to the end of the file or the limit was exceeded
    LimitExceeded,

    /// Not a class file, the header does not equal 0xCAFEBABE
    NotAClass,

    /// Invalid constant pool entry
    InvalidCPItem(u16),

    /// The constant pool cannot be larger than `u16::max_value()`
    CPTooLarge,

    /// Invalid instruction, (e.g. unknown op code)
    InvalidInstruction { op_code: u8, at: u32 },

    /// Reserved (invalid) stack map frame
    ReservedStackMapFrame(u8),

    /// Invalid verification type in stack map table
    InvalidVerificationType(u8),

    /// Invalid element value of annotation, where the u8 is the tag
    InvalidElementValue(u8),

    /// Invalid target type of annotation
    InvalidTargetType,

    /// Invalid type path element kind of annotation
    InvalidTypePath,
}

pub type Result<T> = result::Result<T, Error>;
