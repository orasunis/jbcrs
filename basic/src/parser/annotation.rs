use super::*;

/// Reads the next few parameter annotations.
pub fn parse_parameter_annotations(decoder: &mut Decoder) -> Result<Vec<Vec<Annotation>>> {
    let count = decoder.read_u8()?;
    let mut annotations = Vec::with_capacity(count as usize);
    for _ in 0..count {
        annotations.push(parse_annotations(decoder)?);
    }
    Ok(annotations)
}

/// Reads the next few annotations.
pub fn parse_annotations(decoder: &mut Decoder) -> Result<Vec<Annotation>> {
    let count = decoder.read_u16()?;
    let mut annotations = Vec::with_capacity(count as usize);
    for _ in 0..count {
        annotations.push(parse_annotation(decoder)?);
    }
    Ok(annotations)
}

/// Reads the next few type annotations.
pub fn parse_type_annotations(decoder: &mut Decoder) -> Result<Vec<TypeAnnotation>> {
    let count = decoder.read_u16()?;
    let mut annotations = Vec::with_capacity(count as usize);
    for _ in 0..count {
        annotations.push(parse_type_annotation(decoder)?);
    }
    Ok(annotations)
}

/// Reads a single annotation.
fn parse_annotation(decoder: &mut Decoder) -> Result<Annotation> {
    let type_index = decoder.read_u16()?;
    let count = decoder.read_u16()?;
    let mut element_value_pairs = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let name_index = decoder.read_u16()?;
        element_value_pairs.push((name_index, parse_element_value(decoder)?));
    }

    Ok(Annotation {
        type_index,
        element_value_pairs,
    })
}

/// Reads an element value.
pub fn parse_element_value(decoder: &mut Decoder) -> Result<ElementValue> {
    let tag = decoder.read_u8()?;

    match tag {
        b'B' => Ok(ElementValue::Byte(decoder.read_u16()?)),
        b'S' => Ok(ElementValue::Short(decoder.read_u16()?)),
        b'C' => Ok(ElementValue::Char(decoder.read_u16()?)),
        b'I' => Ok(ElementValue::Int(decoder.read_u16()?)),
        b'J' => Ok(ElementValue::Long(decoder.read_u16()?)),
        b'F' => Ok(ElementValue::Float(decoder.read_u16()?)),
        b'D' => Ok(ElementValue::Double(decoder.read_u16()?)),
        b'Z' => Ok(ElementValue::Boolean(decoder.read_u16()?)),
        b's' => Ok(ElementValue::String(decoder.read_u16()?)),
        b'c' => Ok(ElementValue::Class(decoder.read_u16()?)),
        b'e' => {
            let type_name = decoder.read_u16()?;
            let const_name = decoder.read_u16()?;
            Ok(ElementValue::Enum {
                type_name,
                const_name,
            })
        }
        b'@' => Ok(ElementValue::Annotation(Box::new(parse_annotation(
            decoder,
        )?))),
        b'[' => {
            let count = decoder.read_u16()?;
            let mut element_values = Vec::with_capacity(count as usize);

            for _ in 0..count {
                element_values.push(parse_element_value(decoder)?);
            }

            Ok(ElementValue::Array(element_values))
        }

        _ => Err(Error::InvalidElementValue(tag)),
    }
}

/// Parses a type annotation
fn parse_type_annotation(decoder: &mut Decoder) -> Result<TypeAnnotation> {
    let target_type = parse_target_type(decoder)?;
    let target_path = parse_type_path(decoder)?;
    let annotation = parse_annotation(decoder)?;
    Ok(TypeAnnotation {
        target_type,
        target_path,
        annotation,
    })
}

/// Parses the target type of a type annotation
fn parse_target_type(decoder: &mut Decoder) -> Result<TargetType> {
    use self::TargetType::*;

    Ok(match decoder.read_u8()? {
        0x00 => TypeParameterClass(decoder.read_u8()?),
        0x01 => TypeParameterMethod(decoder.read_u8()?),
        0x10 => SuperType(decoder.read_u16()?),
        0x11 => TypeParameterBoundClass {
            type_parameter: decoder.read_u8()?,
            bound_index: decoder.read_u8()?,
        },
        0x12 => TypeParameterBoundMethod {
            type_parameter: decoder.read_u8()?,
            bound_index: decoder.read_u8()?,
        },
        0x13 => EmptyField,
        0x14 => EmptyReturn,
        0x15 => EmptyReceiver,
        0x16 => FormalParameter(decoder.read_u8()?),
        0x17 => Throws(decoder.read_u16()?),
        0x40 => LocalVariable(parse_local_variable(decoder)?),
        0x41 => ResourceVariable(parse_local_variable(decoder)?),
        0x42 => Catch(decoder.read_u16()?),
        0x43 => OffsetInstanceOf(decoder.read_u16()?),
        0x44 => OffsetNew(decoder.read_u16()?),
        0x45 => OffsetNewRef(decoder.read_u16()?),
        0x46 => OffsetRef(decoder.read_u16()?),
        0x47 => TypeArgumentCast {
            offset: decoder.read_u16()?,
            type_argument: decoder.read_u8()?,
        },
        0x48 => TypeArgumentConstructor {
            offset: decoder.read_u16()?,
            type_argument: decoder.read_u8()?,
        },
        0x49 => TypeArgumentMethod {
            offset: decoder.read_u16()?,
            type_argument: decoder.read_u8()?,
        },
        0x4A => TypeArgumentNewRef {
            offset: decoder.read_u16()?,
            type_argument: decoder.read_u8()?,
        },
        0x4B => TypeArgumentRef {
            offset: decoder.read_u16()?,
            type_argument: decoder.read_u8()?,
        },

        _ => return Err(Error::InvalidTargetType),
    })
}

/// Parses the local variables of a local variable target type
fn parse_local_variable(decoder: &mut Decoder) -> Result<Vec<LocalVariableTarget>> {
    let length = decoder.read_u8()?;
    let mut table = Vec::with_capacity(length as usize);

    for _ in 0..length {
        let start = decoder.read_u16()?;
        let length = decoder.read_u16()?;
        let index = decoder.read_u16()?;
        table.push(LocalVariableTarget {
            start,
            length,
            index,
        })
    }

    Ok(table)
}

/// Parses the type path of a type annotation
fn parse_type_path(decoder: &mut Decoder) -> Result<Vec<TypePathElement>> {
    let length = decoder.read_u8()?;
    let mut type_path = Vec::with_capacity(length as usize);

    for _ in 0..length {
        let path_kind = match decoder.read_u8()? {
            0x00 => TypePathKind::ArrayType,
            0x01 => TypePathKind::NestedType,
            0x02 => TypePathKind::WildcardType,
            0x03 => TypePathKind::Type,

            _ => return Err(Error::InvalidTypePath),
        };

        let argument_index = decoder.read_u8()?;
        type_path.push(TypePathElement {
            path_kind,
            argument_index,
        })
    }

    Ok(type_path)
}
