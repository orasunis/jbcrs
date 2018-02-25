mod encode;

use result::*;
use super::constpool::*;
use super::tree::*;
use self::encode::Encoder;

/// Writes a constant pool and class to a byte vector
pub fn write(constant_pool: &Pool, class: &Class) -> Result<Vec<u8>> {
    let mut encoder = Encoder::new();

    // write magic and version
    encoder.write_bytes(MAGIC);
    encoder.write_u16(class.minor_version);
    encoder.write_u16(class.major_version);

    write_constant_pool(&mut encoder, constant_pool);

    encoder.write_u16(class.access_flags.bits());
    encoder.write_u16(class.name);
    encoder.write_u16(class.super_name);

    encoder.write_u16(class.interfaces.len() as u16);
    for interface in &class.interfaces {
        encoder.write_u16(*interface);
    }

    write_fields(&mut encoder, &class.fields);
    write_methods(&mut encoder, &class.methods);

    write_attributes(&mut encoder, &class.attributes);

    Ok(encoder.bytes())
}

/// Writes the constant pool
fn write_constant_pool(encoder: &mut Encoder, pool: &Pool) {
    // write length and after that the items
    encoder.write_u16(pool.encoded_length());
    for item in pool.get_items() {
        match *item {
            Item::UTF8(ref s) => {
                encoder.write_u8(1);
                encoder.write_u16(s.len() as u16);
                encoder.write_str(s.as_ref());
            }
            Item::Integer(value) => {
                encoder.write_u8(3);
                encoder.write_i32(value);
            }
            Item::Float(value) => {
                encoder.write_u8(4);
                encoder.write_f32(value);
            }
            Item::Long(value) => {
                encoder.write_u8(5);
                encoder.write_i64(value);
            }
            Item::Double(value) => {
                encoder.write_u8(6);
                encoder.write_f64(value);
            }
            Item::Class(class) => {
                encoder.write_u8(7);
                encoder.write_u16(class);
            }
            Item::String(class) => {
                encoder.write_u8(8);
                encoder.write_u16(class);
            }
            Item::FieldRef {
                class,
                name_and_type,
            } => {
                encoder.write_u8(9);
                encoder.write_u16(class);
                encoder.write_u16(name_and_type);
            }
            Item::MethodRef {
                class,
                name_and_type,
            } => {
                encoder.write_u8(10);
                encoder.write_u16(class);
                encoder.write_u16(name_and_type);
            }
            Item::InterfaceMethodRef {
                class,
                name_and_type,
            } => {
                encoder.write_u8(11);
                encoder.write_u16(class);
                encoder.write_u16(name_and_type);
            }
            Item::NameAndType { name, desc } => {
                encoder.write_u8(12);
                encoder.write_u16(name);
                encoder.write_u16(desc);
            }
            Item::MethodHandle { ref kind, index } => {
                use self::ReferenceKind::*;

                encoder.write_u8(15);
                encoder.write_u8(match *kind {
                    GetField => 1,
                    GetStatic => 2,
                    PutField => 3,
                    PutStatic => 4,
                    InvokeVirtual => 5,
                    InvokeStatic => 6,
                    InvokeSpecial => 7,
                    NewInvokeSpecial => 8,
                    InvokeInterface => 9,
                });
                encoder.write_u16(index);
            }
            Item::MethodType(index) => {
                encoder.write_u8(16);
                encoder.write_u16(index);
            }
            Item::InvokeDynamic {
                bootstrap_method,
                name_and_type,
            } => {
                encoder.write_u8(18);
                encoder.write_u16(bootstrap_method);
                encoder.write_u16(name_and_type);
            }
            Item::Module(index) => {
                encoder.write_u8(19);
                encoder.write_u16(index);
            }
            Item::Package(index) => {
                encoder.write_u8(20);
                encoder.write_u16(index);
            }
        }
    }
}

/// Writes all fields to the encoder
fn write_fields(encoder: &mut Encoder, fields: &[Field]) {
    encoder.write_u16(fields.len() as u16);
    for field in fields {
        encoder.write_u16(field.access_flags.bits());
        encoder.write_u16(field.name);
        encoder.write_u16(field.desc);
        write_attributes(encoder, &field.attributes);
    }
}

/// Writes all methods to the encoder
fn write_methods(encoder: &mut Encoder, methods: &[Method]) {
    encoder.write_u16(methods.len() as u16);
    for method in methods {
        encoder.write_u16(method.access_flags.bits());
        encoder.write_u16(method.name);
        encoder.write_u16(method.desc);
        write_attributes(encoder, &method.attributes);
    }
}

/// Writes all attributes to the encoder
fn write_attributes(encoder: &mut Encoder, _attributes: &[Attribute]) {
    // implement later
    encoder.write_u16(0);
}
