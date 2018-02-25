mod decode;
mod class;
mod method;
mod code;
mod annotation;

pub use super::constpool::*;
pub use super::tree::*;
pub use result::*;

use self::class::*;
use self::method::*;
use self::code::*;
use self::annotation::*;
use self::decode::Decoder;

/// The first 4 bytes of every java class file
const MAGIC: &[u8] = &[0xCA, 0xFE, 0xBA, 0xBE];

/// Parses the class file, which is represented as a byte array.
/// The constant pool and the class is returned, if no error occurred.
pub fn parse(input: &[u8]) -> Result<(Pool, Class)> {
    // create a new decoder from the byte array
    let mut cursor = 0;
    let mut decoder = Decoder::new(input, &mut cursor);

    // check if input is a class file
    if decoder.read_bytes(4)? != MAGIC {
        return Err(Error::NotAClass);
    }

    let minor_version = decoder.read_u16()?;
    let major_version = decoder.read_u16()?;

    let constant_pool = read_constant_pool(&mut decoder)?;

    let access_flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);

    let name = decoder.read_u16()?;
    let super_name = decoder.read_u16()?;

    // Read interfaces
    let interface_count = decoder.read_u16()?;
    let mut interfaces = Vec::with_capacity(interface_count as usize);
    for _ in 0..interface_count {
        interfaces.push(decoder.read_u16()?);
    }

    let fields = parse_fields(&mut decoder, &constant_pool)?;
    let methods = parse_methods(&mut decoder, &constant_pool)?;
    let attributes = parse_attributes(&mut decoder, &constant_pool)?;

    let class = Class {
        minor_version,
        major_version,

        access_flags,
        name,
        super_name,

        interfaces,

        fields,
        methods,
        attributes,
    };

    Ok((constant_pool, class))
}

/// Reads the entire constant pool
fn read_constant_pool(decoder: &mut Decoder) -> Result<Pool> {
    let size = decoder.read_u16()?;
    let mut pool = Pool::with_capacity(size);

    for index in 1..size {
        let tag = decoder.read_u8()?;

        // match a tag and read the additional information
        let item = match tag {
            1 => {
                let length = decoder.read_u16()?;
                Item::UTF8(decoder.read_str(length as usize)?)
            }
            3 => Item::Integer(decoder.read_i32()?),
            4 => Item::Float(decoder.read_f32()?),
            5 => Item::Long(decoder.read_i64()?),
            6 => Item::Double(decoder.read_f64()?),
            7 => Item::Class(decoder.read_u16()?),
            8 => Item::String(decoder.read_u16()?),
            9 => {
                let class = decoder.read_u16()?;
                let name_and_type = decoder.read_u16()?;

                Item::FieldRef {
                    class,
                    name_and_type,
                }
            }
            10 => {
                let class = decoder.read_u16()?;
                let name_and_type = decoder.read_u16()?;

                Item::MethodRef {
                    class,
                    name_and_type,
                }
            }
            11 => {
                let class = decoder.read_u16()?;
                let name_and_type = decoder.read_u16()?;

                Item::InterfaceMethodRef {
                    class,
                    name_and_type,
                }
            }
            12 => {
                let name = decoder.read_u16()?;
                let desc = decoder.read_u16()?;

                Item::NameAndType { name, desc }
            }
            15 => {
                let kind = match decoder.read_u8()? {
                    1 => ReferenceKind::GetField,
                    2 => ReferenceKind::GetStatic,
                    3 => ReferenceKind::PutField,
                    4 => ReferenceKind::PutStatic,
                    5 => ReferenceKind::InvokeVirtual,
                    6 => ReferenceKind::InvokeStatic,
                    7 => ReferenceKind::InvokeSpecial,
                    8 => ReferenceKind::NewInvokeSpecial,
                    9 => ReferenceKind::InvokeInterface,

                    _ => return Err(Error::InvalidCPItem(index)),
                };
                let index = decoder.read_u16()?;

                Item::MethodHandle { kind, index }
            }
            16 => Item::MethodType(decoder.read_u16()?),
            18 => {
                let bootstrap_method_attribute_index = decoder.read_u16()?;
                let name_and_type_index = decoder.read_u16()?;

                Item::InvokeDynamic {
                    bootstrap_method_attribute: bootstrap_method_attribute_index,
                    name_and_type: name_and_type_index,
                }
            }
            19 => Item::Module(decoder.read_u16()?),
            20 => Item::Package(decoder.read_u16()?),

            _ => return Err(Error::InvalidCPItem(index)),
        };

        pool.push_with_dup(Some(item))?;

        // long and double values take two spaces
        if tag == 5 || tag == 6 {
            pool.push_with_dup(None)?;
        }
    }

    Ok(pool)
}

/// Parses all fields and their attributes
fn parse_fields(decoder: &mut Decoder, constant_pool: &Pool) -> Result<Vec<Field>> {
    let count = decoder.read_u16()?;
    let mut fields = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let access_flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);
        let name = decoder.read_u16()?;
        let desc = decoder.read_u16()?;
        let attributes = parse_attributes(decoder, constant_pool)?;

        fields.push(Field {
            access_flags,
            name,
            desc,
            attributes,
        })
    }

    Ok(fields)
}

/// Parses all methods and their attributes
fn parse_methods(decoder: &mut Decoder, constant_pool: &Pool) -> Result<Vec<Method>> {
    let count = decoder.read_u16()?;
    let mut fields = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let access_flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);
        let name = decoder.read_u16()?;
        let desc = decoder.read_u16()?;
        let attributes = parse_attributes(decoder, constant_pool)?;

        fields.push(Method {
            access_flags,
            name,
            desc,
            attributes,
        })
    }

    Ok(fields)
}

/// Parses all attributes
fn parse_attributes(decoder: &mut Decoder, constant_pool: &Pool) -> Result<Vec<Attribute>> {
    let count = decoder.read_u16()?;
    let mut attributes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let name_index = decoder.read_u16()?;
        let name = constant_pool.get_utf8(name_index)?;
        let length = decoder.read_u32()?;

        // limit attribute length
        let mut attr_decoder = decoder.limit(length as usize)?;

        let attribute = match name.as_ref() {
            "AnnotationDefault" => {
                Attribute::AnnotationDefault(parse_element_value(&mut attr_decoder)?)
            }
            "BootstrapMethods" => parse_bootstrap_methods(&mut attr_decoder)?,
            "Code" => parse_code(&mut attr_decoder, constant_pool)?,
            "ConstantValue" => {
                let index = attr_decoder.read_u16()?;
                Attribute::ConstantValue(index)
            }
            "Deprecated" => Attribute::Deprecated,
            "EnclosingMethods" => parse_enclosing_method(&mut attr_decoder)?,
            "Exceptions" => parse_exceptions(&mut attr_decoder)?,
            "InnerClasses" => parse_inner_classes(&mut attr_decoder)?,
            "LineNumberTable" => parse_line_number_table(&mut attr_decoder)?,
            "LocalVariableTable" => parse_local_variable_table(&mut attr_decoder)?,
            "LocalVariableTypeTable" => parse_local_variable_type_table(&mut attr_decoder)?,
            "MethodParameters" => parse_method_parameters(&mut attr_decoder)?,
            "Module" => parse_module(&mut attr_decoder)?,
            "ModuleMainClass" => {
                let index = attr_decoder.read_u16()?;
                Attribute::ModuleMainClass(index)
            }
            "ModulePackages" => parse_module_packages(&mut attr_decoder)?,
            "RuntimeVisibleAnnotations" => {
                let annotations = parse_annotations(&mut attr_decoder)?;
                Attribute::RuntimeVisibleAnnotations(annotations)
            }
            "RuntimeInvisibleAnnotations" => {
                let annotations = parse_annotations(&mut attr_decoder)?;
                Attribute::RuntimeInvisibleAnnotations(annotations)
            }
            "RuntimeVisibleParameterAnnotations" => {
                let annotations = parse_parameter_annotations(&mut attr_decoder)?;
                Attribute::RuntimeVisibleParameterAnnotations(annotations)
            }
            "RuntimeInvisibleParameterAnnotations" => {
                let annotations = parse_parameter_annotations(&mut attr_decoder)?;
                Attribute::RuntimeInvisibleParameterAnnotations(annotations)
            }
            "RuntimeVisibleTypeAnnotations" => {
                let annotations = parse_type_annotations(&mut attr_decoder)?;
                Attribute::RuntimeVisibleTypeAnnotations(annotations)
            }
            "RuntimeInvisibleTypeAnnotations" => {
                let annotations = parse_type_annotations(&mut attr_decoder)?;
                Attribute::RuntimeInvisibleTypeAnnotations(annotations)
            }
            "SourceFile" => {
                let index = attr_decoder.read_u16()?;
                Attribute::SourceFile(index)
            }
            "Signature" => {
                let index = attr_decoder.read_u16()?;
                Attribute::Signature(index)
            }
            "StackMapTable" => parse_stack_map_table(&mut attr_decoder)?,
            "Synthetic" => Attribute::Synthetic,
            "SourceDebugExtension" => {
                Attribute::SourceDebugExtension(attr_decoder.read_str(length as usize)?)
            }

            _ => {
                let bytes = attr_decoder.read_bytes(length as usize)?;
                Attribute::Unknown(name_index, bytes.to_vec())
            }
        };
        attributes.push(attribute);

        // go on
        attr_decoder.remove_limit()?;
    }

    Ok(attributes)
}
