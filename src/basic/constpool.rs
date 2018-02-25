use result::*;

/// A constant pool item
#[derive(PartialEq, Debug)]
pub enum Item {
    /// An UTF-8 encoded string.
    /// Inside the class file itself, a modified format is used.
    UTF8(String),
    /// An `int`.
    Integer(i32),
    /// A `float`.
    Float(f32),
    /// A `long`.
    /// Takes two spots, instead of one.
    Long(i64),
    /// A `double`.
    /// Takes two spots, instead of one.
    Double(f64),
    /// An index to the name of the class, or the descriptor of an array type.
    /// Always refers to an `Item::UTF8(_)`.
    Class(u16),
    /// A `java.lang.String` object.
    String(u16),
    /// Describes a field reference.
    FieldRef {
        /// The index to an `Item::Class(_)`.
        /// Can be either a Class, or an Interface.
        class: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Describes a method reference.
    MethodRef {
        /// The index to an `Item::Class(_)`.
        /// Must be a Class.
        class: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Describes a method reference, where the class is an interface.
    InterfaceMethodRef {
        /// The index to an `Item::Class(_)`.
        /// Must be an Interface.
        class: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Represents a field or method, without indicating which class or type it belongs to.
    NameAndType {
        /// The index to an `Item::UTF8(_)`.
        /// Must either be a valid unqualfied name or `<init>`.
        name: u16,
        /// The index to an `Item::UTF8(_)`.
        /// Represents a valid field or method descriptor.
        desc: u16,
    },
    /// Represents a method handle
    MethodHandle {
        /// Characterizes its bytecode behaviour.
        kind: ReferenceKind,
        /// If kind is either GetField, GetStatic, PutField, or PutStatic,
        /// the entry at that index must be a `Item::FieldRef { .. }`.
        /// If kind is InvokeVirtual or InvokeSpecial,
        /// the entry at that index must be `Item::MethodRef { .. }`.
        /// If kind is InvokeStatic or InvokeSpecial
        /// and the version of the class is less than 52.0,
        /// the entry at that index must be `Item::MethodRef { .. }`.
        /// If it is 52.0 or above,
        /// it must either be a MethodRef or an `Item::InterfaceMethodRef { .. }`.
        /// If kind is InvokeInterface,
        /// the entry at that index must be an `Item::InterfaceMethodRef { .. }`.
        index: u16,
    },
    /// Describes a method type.
    /// The entry at that index must be an `Item::UTF8(_)`
    /// representing a method descriptor.
    MethodType(u16),
    /// Describes a invoke dynamic instruction,
    /// and specifies information regarding the bootstrap method.
    InvokeDynamic {
        /// The index to an entry of the BootstrapMethods attribute of the class file.
        bootstrap_method: u16,
        /// The index to an `Item::NameAndType { .. }`.
        name_and_type: u16,
    },
    /// Represents a module.
    /// The entry at that index must be a `Item::UTF8(_)` with a valid name.
    /// The class must have the MODULE flag set.
    Module(u16),
    /// Represents a package exported or opened by a module.
    /// The entry at that index must be an `Item::UTF8(_)`
    /// with a valid package name encoded in internal form.
    /// The class must have the MODULE flag set.
    Package(u16),
}

impl Item {
    /// Returns true if self consumes two spaces in the constant pool.
    pub fn is_double(&self) -> bool {
        match *self {
            Item::Double(_) | Item::Long(_) => true,
            _ => false,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ReferenceKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}

/// The constant pool found in every java class file.
/// It is used to have fast lookup for entries and small files.
#[derive(Default)]
pub struct Pool {
    /// The constant pool items.
    /// A Option is used, since long and double values take two spaces
    /// and we still want to access items by index using O(1), not O(n).
    items: Vec<Option<Item>>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool { items: Vec::new() }
    }

    pub fn with_capacity(size: u16) -> Pool {
        Pool {
            items: Vec::with_capacity(size as usize),
        }
    }

    /// Returns the encoded length of the table.
    /// Long and double items are included.
    pub fn encoded_length(&self) -> u16 {
        self.items.len() as u16 + 1
    }

    /// Pushes a new item on the pool, returning the index to it.
    /// If the pool size exceeds u16::max_value() an error will be returned.
    /// We won't check for duplicates here since this method should be used when reading
    /// since later items should be accessed by index.
    /// We will also gain performance benefits.
    pub fn push_with_dup(&mut self, item: Option<Item>) -> Result<u16> {
        if self.items.len() == u16::max_value() as usize - 1 {
            Err(Error::CPTooLarge)
        } else {
            self.items.push(item);

            Ok(self.items.len() as u16)
        }
    }

    /// Pushes a new item on the pool, returning the index to it.
    /// If the pool size exceeds u16::max_value() an error will be returned.
    /// If duplicates are found, the duplicate will be used and no item added.
    pub fn push(&mut self, item: Item) -> Result<u16> {
        for (index, it) in self.items.iter().enumerate() {
            if let Some(ref it) = *it {
                if *it == item {
                    return Ok(index as u16 + 1);
                }
            }
        }

        if self.items.len() == u16::max_value() as usize - 1 {
            Err(Error::CPTooLarge)
        } else {
            self.items.push(Some(item));

            Ok(self.items.len() as u16)
        }
    }

    /// Pushes a new UTF-8 item on the pool and returns an index to it.
    pub fn push_utf8(&mut self, content: String) -> Result<u16> {
        self.push(Item::UTF8(content))
    }

    /// Pushes a new class item on the pool and returns an index to it.
    pub fn push_class(&mut self, name: String) -> Result<u16> {
        let name_index = self.push_utf8(name)?;
        self.push(Item::Class(name_index))
    }

    /// Returns a Vector containing pointers to Items.
    ///
    /// The *Nones* inside the items Vec are filtered.
    pub fn get_items(&self) -> Vec<&Item> {
        let mut items = Vec::with_capacity(self.items.len());

        for opt_item in &self.items {
            if let Some(ref item) = *opt_item {
                items.push(item);
            }
        }

        items
    }

    /// Returns the element at a specified index.
    /// If the index is 0 or greater than the size of the pool, an error is returned.
    pub fn get(&self, index: u16) -> Result<&Item> {
        self.items
            .get(index as usize - 1)
            .ok_or(Error::InvalidCPItem(index))?
            .as_ref()
            .ok_or(Error::InvalidCPItem(index))
    }

    /// Returns a cloned String at a specified index.
    pub fn get_utf8(&self, index: u16) -> Result<String> {
        if let Item::UTF8(ref s) = *self.get(index)? {
            Ok(s.clone())
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }

    /// Returns a class name at a specified index,
    /// but if the utf index is 0, None is returned.
    pub fn get_class_name_opt(&self, index: u16) -> Result<Option<String>> {
        if let Item::Class(utf_index) = *self.get(index)? {
            if utf_index == 0 {
                Ok(None)
            } else {
                Ok(Some(self.get_utf8(utf_index)?))
            }
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }

    /// Returns a class name at a specified index.
    pub fn get_class_name(&self, index: u16) -> Result<String> {
        if let Item::Class(utf_index) = *self.get(index)? {
            self.get_utf8(utf_index)
        } else {
            Err(Error::InvalidCPItem(index))
        }
    }
}
