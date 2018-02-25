use result::*;

use std::str::FromStr;
use std::fmt::{self, Write};

/// All types present in a type descriptor.
#[derive(Eq, PartialEq, Debug)]
pub enum Type {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
    Reference(String),
}

/// A `TypeDescriptor` is either a field descriptor,
/// a single type (parameter or return type) of a method,
/// or an element value of an annotation.
/// In the JVM Specification `FieldDescriptor` is used as a name.
/// Maybe using that one would be better, but I am too lazy to refactor now.
#[derive(Eq, PartialEq, Debug)]
pub struct TypeDescriptor {
    /// The dimensions of the type
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::TypeDescriptor;
    /// let desc: TypeDescriptor = "[[I".parse().unwrap();
    /// assert_eq!(desc.dimensions, 2);
    /// ```
    pub dimensions: u8,

    /// The base type
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, TypeDescriptor};
    ///
    /// let short_desc: TypeDescriptor = "S".parse().unwrap();
    /// assert_eq!(short_desc.base_type, Type::Short);
    ///
    /// let string_desc: TypeDescriptor = "[Ljava/lang/String;".parse().unwrap();
    /// assert_eq!(
    ///     string_desc.base_type,
    ///     Type::Reference("java/lang/String".to_owned())
    /// );
    /// ```
    pub base_type: Type,
}

impl TypeDescriptor {
    pub fn new(dimensions: u8, base_type: Type) -> TypeDescriptor {
        TypeDescriptor {
            dimensions,
            base_type,
        }
    }
}

impl FromStr for TypeDescriptor {
    type Err = Error;

    /// Parses a string and returns a TypeDescriptor if it succeeded.
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, TypeDescriptor};
    ///
    /// let desc: TypeDescriptor = "[[[D".parse().unwrap();
    /// assert_eq!(desc, TypeDescriptor { dimensions: 3, base_type: Type::Double });
    /// ```
    fn from_str(desc: &str) -> Result<TypeDescriptor> {
        // read the string char by char, not bytes
        let mut chars = desc.chars();
        // the array dimensions of the type
        let mut dimensions: u8 = 0;
        // the current location in the descriptor,
        // used to generate better errors
        let mut i = 0;

        // avoid code duplication
        macro_rules! err {
            () => {{
                return Err(Error::InvalidDescriptor {
                    desc: desc.to_owned(),
                    at: i,
                });
            }}
        }

        // read array dimensions and type tag
        loop {
            let ch = match chars.next() {
                Some(ch) => ch,
                None => err!(),
            };

            if ch == '[' {
                // no more than 255 array dimensions are allowed
                dimensions = match dimensions.checked_add(1) {
                    Some(d) => d,
                    None => err!(),
                };
                i += 1;
            } else if ch == 'L' {
                // read name of reference
                break;
            } else {
                // after the primitive tag no chars may be
                if chars.count() != 0 {
                    err!();
                }

                // primitive types can be returned now
                return Ok(TypeDescriptor::new(
                    dimensions,
                    match ch {
                        'Z' => Type::Boolean,
                        'B' => Type::Byte,
                        'S' => Type::Short,
                        'I' => Type::Int,
                        'J' => Type::Long,
                        'F' => Type::Float,
                        'D' => Type::Double,
                        'C' => Type::Char,
                        _ => err!(), // unknown type tag
                    },
                ));
            }
        }

        // A reference descriptor is made up of:
        // [dimensions] L [name] ;
        // There is no need to have a shorter name than this
        // usize.max(usize) is used to prevent panics
        let mut name = String::with_capacity((desc.len() - dimensions as usize).max(2) - 2);

        // now read the name of the reference
        loop {
            let ch = match chars.next() {
                Some(ch) => ch,
                None => err!(),
            };
            i += 1;
            if ch == ';' {
                // a class name cannot be empty, nor can any chars follow the descriptor
                if name.is_empty() || chars.count() != 0 {
                    err!();
                }

                return Ok(TypeDescriptor::new(dimensions, Type::Reference(name)));
            } else {
                name.push(ch);
            }
        }
    }
}

impl fmt::Display for TypeDescriptor {
    /// Formats this descriptor
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, TypeDescriptor};
    ///
    /// let mut desc: TypeDescriptor = "[[Ljava/lang/String;".parse().unwrap();
    /// desc.base_type = Type::Float;
    /// assert_eq!("[[F", desc.to_string());
    ///
    /// desc.base_type = Type::Reference("java/lang/Float".to_owned());
    /// assert_eq!("[[Ljava/lang/Float;", desc.to_string());
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // dimensions
        f.write_str(&"[".repeat(self.dimensions as usize))?;

        // base type
        match self.base_type {
            Type::Boolean => f.write_char('Z'),
            Type::Byte => f.write_char('B'),
            Type::Short => f.write_char('S'),
            Type::Int => f.write_char('I'),
            Type::Long => f.write_char('J'),
            Type::Float => f.write_char('F'),
            Type::Double => f.write_char('D'),
            Type::Char => f.write_char('C'),
            Type::Reference(ref name) => write!(f, "L{};", name),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct MethodDescriptor {
    /// The parameter types of the method.
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, MethodDescriptor};
    ///
    /// let desc: MethodDescriptor = "(Z)V".parse().unwrap();
    /// assert_eq!(Type::Boolean, desc.params[0].base_type);
    /// ```
    pub params: Vec<TypeDescriptor>,

    /// The return type of the method.
    /// None indicates 'void'
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, MethodDescriptor};
    ///
    /// let desc: MethodDescriptor = "()I".parse().unwrap();
    /// assert_eq!(Type::Int, desc.return_type.unwrap().base_type);
    /// ```
    pub return_type: Option<TypeDescriptor>,
}

impl MethodDescriptor {
    pub fn new(
        params: Vec<TypeDescriptor>,
        return_type: Option<TypeDescriptor>,
    ) -> MethodDescriptor {
        MethodDescriptor {
            params,
            return_type,
        }
    }
}

impl FromStr for MethodDescriptor {
    type Err = Error;

    /// Parses a string and returns a MethodDescriptor if it succeeded.
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, TypeDescriptor, MethodDescriptor};
    ///
    /// let desc: MethodDescriptor = "([[DLjava/lang/Integer;)V".parse().unwrap();
    /// assert_eq!(desc, MethodDescriptor {
    ///     params: vec![
    ///         TypeDescriptor {
    ///             dimensions: 2,
    ///             base_type: Type::Double,
    ///         },
    ///         TypeDescriptor {
    ///             dimensions: 0,
    ///             base_type: Type::Reference("java/lang/Integer".to_owned()),
    ///         },
    ///     ],
    ///     return_type: None,
    /// });
    /// ```
    fn from_str(desc: &str) -> Result<MethodDescriptor> {
        let mut chars = desc.chars();
        let mut i = 0;

        // avoid code duplication
        macro_rules! err {
            () => {{
                return Err(Error::InvalidDescriptor {
                    desc: desc.to_owned(),
                    at: i,
                });
            }}
        }

        match chars.next() {
            Some('(') => {}
            _ => err!(),
        }

        let mut params = Vec::new();
        let mut ret = None;
        let mut state = 0;

        // sorry for this messy code
        'type_loop: loop {
            let mut dimensions: u8 = 0;

            // read array dimensions and type tag
            for ch in &mut chars {
                i += 1;
                if ch == '[' {
                    // no more than 255 array dimensions are allowed
                    dimensions = match dimensions.checked_add(1) {
                        Some(d) => d,
                        None => err!(),
                    };
                } else if state == 0 && ch == ')' {
                    if dimensions != 0 {
                        err!();
                    }
                    state = 1;
                } else if state == 1 && ch == 'V' {
                    if dimensions != 0 {
                        err!();
                    }

                    break 'type_loop;
                } else {
                    let parsed_desc = TypeDescriptor::new(
                        dimensions,
                        match ch {
                            'Z' => Type::Boolean,
                            'B' => Type::Byte,
                            'S' => Type::Short,
                            'I' => Type::Int,
                            'J' => Type::Long,
                            'F' => Type::Float,
                            'D' => Type::Double,
                            'C' => Type::Char,
                            'L' => break, // read the name of the reference
                            _ => err!(),  // unknown type tag
                        },
                    );

                    if state == 0 {
                        params.push(parsed_desc);
                        continue 'type_loop;
                    } else {
                        ret = Some(parsed_desc);
                        break 'type_loop;
                    }
                }
            }

            let mut name = String::new();

            // now read the name of the reference
            for ch in &mut chars {
                i += 1;
                if ch == ';' {
                    if name.is_empty() {
                        err!();
                    }

                    let parsed_desc = TypeDescriptor::new(dimensions, Type::Reference(name));
                    if state == 0 {
                        params.push(parsed_desc);
                        continue 'type_loop;
                    } else {
                        ret = Some(parsed_desc);
                        break 'type_loop;
                    }
                } else {
                    name.push(ch);
                }
            }

            err!();
        }

        if chars.count() != 0 || params.len() > 255 {
            err!();
        }

        Ok(MethodDescriptor::new(params, ret))
    }
}

impl fmt::Display for MethodDescriptor {
    /// Formats this descriptor
    ///
    /// # Examples
    ///
    /// ```
    /// use jbcrs::{Type, MethodDescriptor};
    ///
    /// let mut desc: MethodDescriptor = "(Ljava/lang/String;)I".parse().unwrap();
    /// desc.return_type.as_mut().unwrap().base_type = Type::Long;
    /// assert_eq!("(Ljava/lang/String;)J", desc.to_string());
    ///
    /// desc.params[0].base_type = Type::Reference("java/lang/Double".to_owned());
    /// assert_eq!("(Ljava/lang/Double;)J", desc.to_string());
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char('(')?;
        for param in &self.params {
            param.fmt(f)?;
        }
        f.write_char(')')?;
        if let Some(ref ret) = self.return_type {
            ret.fmt(f)
        } else {
            f.write_char('V')
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn td_parse() {
        fn parse(s: &str) -> Result<TypeDescriptor> {
            s.parse()
        }

        assert_eq!(parse("I").unwrap(), TypeDescriptor::new(0, Type::Int));
        assert_eq!(parse("[[B").unwrap(), TypeDescriptor::new(2, Type::Byte));
        assert_eq!(
            parse("[Ljava/lang/String;").unwrap(),
            TypeDescriptor::new(1, Type::Reference("java/lang/String".to_owned()))
        );
        assert_eq!(
            parse(format!("{}I", "[".repeat(255)).as_ref()).unwrap(),
            TypeDescriptor::new(255, Type::Int)
        );

        // errors
        assert!(parse("U").is_err()); // unknown tag
        assert!(parse("IJ").is_err()); // multiple tags
        assert!(parse("I[").is_err()); // array at wrong location
        assert!(parse("Ljava/lang/String").is_err()); // no trailing semicolon
        assert!(parse("Ljava/lang/String;;").is_err()); // too many trailing semicolons
        assert!(parse("L;").is_err()); // empty name
        assert!(parse("L").is_err()); // empty name and no semicolon
        assert!(parse("Ljava/lang/String;I").is_err()); // multiple tags but with reference
        assert!(parse(format!("{}I", "[".repeat(256)).as_ref()).is_err()); // too many dimensions
    }

    #[test]
    fn md_parse() {
        fn parse(s: &str) -> Result<MethodDescriptor> {
            s.parse()
        }

        assert_eq!(
            parse("()V").unwrap(),
            MethodDescriptor::new(Vec::new(), None)
        );
        assert_eq!(
            parse("()[J").unwrap(),
            MethodDescriptor::new(Vec::new(), Some(TypeDescriptor::new(1, Type::Long)))
        );
        assert_eq!(
            parse("([[Ljava/lang/String;I)V").unwrap(),
            MethodDescriptor::new(
                vec![
                    TypeDescriptor::new(2, Type::Reference("java/lang/String".to_owned())),
                    TypeDescriptor::new(0, Type::Int),
                ],
                None
            )
        );
        assert!(parse("(U)V").is_err()); // unknown tag
        assert!(parse("()U").is_err()); // unknown tag
        assert!(parse("(V)V").is_err()); // void as argument
        assert!(parse("()IJ").is_err()); // multiple return types
        assert!(parse("(I[)V").is_err()); // array at wrong location
        assert!(parse("()Ljava/lang/String").is_err()); // no trailing semicolon
        assert!(parse("(Ljava/lang/String;;)V").is_err()); // too many trailing semicolons
        assert!(parse("(L;)V").is_err()); // empty name
        assert!(parse("(L)V").is_err()); // empty name and no semicolon
        assert!(parse("()L;").is_err()); // empty name
        assert!(parse("())V").is_err()); // multiple closing braces
        assert!(parse("(()V").is_err()); // multiple opening braces
        assert!(parse("{)V").is_err()); // invalid opening brace
        assert!(parse("(V").is_err()); // no closing brace
        assert!(parse(")V").is_err()); // no opening brace
        assert!(parse("I").is_err()); // not a method descriptor
        assert!(parse("").is_err()); // empty
        assert!(parse("()[[V").is_err()); // multidimensional void
        assert!(parse(format!("({}I)V", "[".repeat(256)).as_ref()).is_err()); // too many dimensions
        assert!(parse(format!("({})V", "I".repeat(256)).as_ref()).is_err()); // too many parameters
    }

}
