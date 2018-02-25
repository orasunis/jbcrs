use super::*;
use self::decode::Decoder;

/// Parses the `Exceptions` attribute.
pub fn parse_exceptions(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut exceptions = Vec::with_capacity(count as usize);
    for _ in 0..count {
        exceptions.push(decoder.read_u16()?);
    }
    Ok(Attribute::Exceptions(exceptions))
}

/// Parses the `LineNumberTable` attribute.
pub fn parse_line_number_table(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut table = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let start = decoder.read_u16()?;
        let line_number = decoder.read_u16()?;
        table.push(LineNumber { start, line_number })
    }
    Ok(Attribute::LineNumberTable(table))
}

/// Parses the `LocalVariableTable` attribute.
pub fn parse_local_variable_table(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut table = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let start = decoder.read_u16()?;
        let length = decoder.read_u16()?;
        let name = decoder.read_u16()?;
        let descriptor = decoder.read_u16()?;
        let index = decoder.read_u16()?;
        table.push(LocalVariable {
            start,
            length,
            name,
            descriptor,
            index,
        });
    }
    Ok(Attribute::LocalVariableTable(table))
}

/// Parses the `LocalVariableTypeTable` attribute.
pub fn parse_local_variable_type_table(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut table = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let start = decoder.read_u16()?;
        let length = decoder.read_u16()?;
        let name = decoder.read_u16()?;
        let signature = decoder.read_u16()?;
        let index = decoder.read_u16()?;
        table.push(LocalVariableType {
            start,
            length,
            name,
            signature,
            index,
        });
    }
    Ok(Attribute::LocalVariableTypeTable(table))
}

/// Parses the `StackMapTable` attribute.
pub fn parse_stack_map_table(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut table = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let frame_type = decoder.read_u8()?;
        let frame = match frame_type {
            0...63 => StackMapFrame::Same {
                offset_delta: u16::from(frame_type),
            },
            64...127 => StackMapFrame::Same1 {
                offset_delta: u16::from(frame_type) - 64,
                stack: parse_verification_type(decoder)?,
            },
            247 => StackMapFrame::Same1 {
                offset_delta: decoder.read_u16()?,
                stack: parse_verification_type(decoder)?,
            },
            248...250 => StackMapFrame::Chop {
                offset_delta: decoder.read_u16()?,
                count: 251 - frame_type,
            },
            251 => StackMapFrame::Same {
                offset_delta: decoder.read_u16()?,
            },
            252...254 => {
                let offset_delta = decoder.read_u16()?;
                let dif = frame_type as usize - 251;
                let mut locals = Vec::with_capacity(dif);
                for _ in 0..dif {
                    locals.push(parse_verification_type(decoder)?);
                }

                StackMapFrame::Append {
                    offset_delta,
                    locals,
                }
            }
            255 => {
                let offset_delta = decoder.read_u16()?;
                let local_count = decoder.read_u16()? as usize;
                let mut locals = Vec::with_capacity(local_count);
                for _ in 0..local_count {
                    locals.push(parse_verification_type(decoder)?);
                }

                let stack_size = decoder.read_u16()? as usize;
                let mut stack = Vec::with_capacity(stack_size);
                for _ in 0..stack_size {
                    stack.push(parse_verification_type(decoder)?);
                }

                StackMapFrame::Full {
                    offset_delta,
                    locals,
                    stack,
                }
            }

            _ => return Err(Error::ReservedStackMapFrame(frame_type)),
        };
        table.push(frame);
    }

    Ok(Attribute::StackMapTable(table))
}

/// Parses a verification type.
fn parse_verification_type(decoder: &mut Decoder) -> Result<VerificationType> {
    use self::VerificationType::*;

    let tag = decoder.read_u8()?;
    match tag {
        0 => Ok(Top),
        1 => Ok(Integer),
        2 => Ok(Float),
        3 => Ok(Double),
        4 => Ok(Long),
        5 => Ok(Null),
        6 => Ok(UninitializedThis),
        7 => Ok(Object(decoder.read_u16()?)),
        8 => Ok(Uninitialized(decoder.read_u16()?)),

        _ => Err(Error::InvalidVerificationType(tag)),
    }
}

/// Parses the `MethodParameters` attribute.
pub fn parse_method_parameters(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut params = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let name = decoder.read_u16()?;
        let access_flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);
        params.push(MethodParameter { name, access_flags });
    }
    Ok(Attribute::MethodParameters(params))
}
