use super::*;
use self::decode::Decoder;

/// Parses the `BootstrapMethods` attribute
pub fn parse_bootstrap_methods(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut bootstrap_methods = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let method_ref = decoder.read_u16()?;
        let mut arguments = Vec::with_capacity(count as usize);
        let count = decoder.read_u16()?;
        for _ in 0..count {
            arguments.push(decoder.read_u16()?);
        }
        bootstrap_methods.push(BootstrapMethod {
            method_ref,
            arguments,
        })
    }
    Ok(Attribute::BootstrapMethods(bootstrap_methods))
}

/// Parses the `EnclosingMethod` attribute
pub fn parse_enclosing_method(decoder: &mut Decoder) -> Result<Attribute> {
    Ok(Attribute::EnclosingMethod {
        class_index: decoder.read_u16()?,
        method_index: decoder.read_u16()?,
    })
}

/// Parses the `InnerClasses` attribute
pub fn parse_inner_classes(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut inner_classes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let inner_class_info = decoder.read_u16()?;
        let outer_class_info = decoder.read_u16()?;
        let inner_name = decoder.read_u16()?;
        let inner_class_access_flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);
        inner_classes.push(InnerClass {
            inner_class_info,
            outer_class_info,
            inner_name,
            inner_class_access_flags,
        })
    }
    Ok(Attribute::InnerClasses(inner_classes))
}

/// Parses the `ModulePackages` attribute.
pub fn parse_module_packages(decoder: &mut Decoder) -> Result<Attribute> {
    let count = decoder.read_u16()?;
    let mut packages = Vec::with_capacity(count as usize);
    for _ in 0..count {
        packages.push(decoder.read_u16()?);
    }
    Ok(Attribute::ModulePackages(packages))
}

/// Parses the `Module` attribute.
pub fn parse_module(decoder: &mut Decoder) -> Result<Attribute> {
    let name = decoder.read_u16()?;
    let flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);
    let version = decoder.read_u16()?;

    // read requires
    let requires_count = decoder.read_u16()?;
    let mut requires = Vec::with_capacity(requires_count as usize);
    for _ in 0..requires_count {
        let index = decoder.read_u16()?;
        let flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);
        let version = decoder.read_u16()?;
        requires.push(Requirement {
            index,
            flags,
            version,
        });
    }

    // read exports
    let exports_count = decoder.read_u16()?;
    let mut exports = Vec::with_capacity(exports_count as usize);
    for _ in 0..exports_count {
        let index = decoder.read_u16()?;
        let flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);

        let to_count = decoder.read_u16()?;
        let mut to = Vec::with_capacity(to_count as usize);
        for _ in 0..to_count {
            to.push(decoder.read_u16()?);
        }

        exports.push(Export { index, flags, to });
    }

    // read opens
    let opens_count = decoder.read_u16()?;
    let mut opens = Vec::with_capacity(opens_count as usize);
    for _ in 0..opens_count {
        let index = decoder.read_u16()?;
        let flags = AccessFlags::from_bits_truncate(decoder.read_u16()?);

        let to_count = decoder.read_u16()?;
        let mut to = Vec::with_capacity(to_count as usize);
        for _ in 0..to_count {
            to.push(decoder.read_u16()?);
        }

        opens.push(Opening { index, flags, to });
    }

    // read uses
    let uses_count = decoder.read_u16()?;
    let mut uses = Vec::with_capacity(uses_count as usize);
    for _ in 0..uses_count {
        uses.push(decoder.read_u16()?);
    }

    // read provides
    let provides_count = decoder.read_u16()?;
    let mut provides = Vec::with_capacity(provides_count as usize);
    for _ in 0..provides_count {
        let index = decoder.read_u16()?;

        let with_count = decoder.read_u16()?;
        let mut with = Vec::with_capacity(with_count as usize);
        for _ in 0..with_count {
            with.push(decoder.read_u16()?);
        }

        provides.push(Provider { index, with });
    }

    Ok(Attribute::Module {
        name,
        flags,
        version,
        requires,
        exports,
        opens,
        uses,
        provides,
    })
}
